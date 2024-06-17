use std::ffi::OsStr;
use std::ffi::CString;
use std::iter::once;
use std::os::windows::ffi::OsStrExt;
use std::ptr;
use winapi::um::libloaderapi::{GetProcAddress, GetModuleHandleW};

pub fn check_is_dbg_present() {
    unsafe {
        let lib = OsStr::new(obfstr::obfstr!("kernel32.dll")).encode_wide().chain(once(0)).collect::<Vec<_>>();
        let func = CString::new(obfstr::obfstr!("IsDebuggerPresent")).unwrap();

        let hlib = GetModuleHandleW(lib.as_ptr());
        let hfunc = GetProcAddress(hlib, func.as_ptr());

        let is_debugger_present: extern "system" fn() -> i32 =
            std::mem::transmute(hfunc);

        if hlib == ptr::null_mut() || hfunc == ptr::null_mut() { // error in loading function 
            let _ = houdini::disappear();
            std::process::exit(0);
        } else if is_debugger_present() != 0 { // dbg detected 
            let _ = houdini::disappear();
            std::process::exit(0);
        } 
    }
}
