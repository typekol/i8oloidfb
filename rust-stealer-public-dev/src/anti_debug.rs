use winapi::um::debugapi::{IsDebuggerPresent, CheckRemoteDebuggerPresent};
use winapi::um::processthreadsapi::GetCurrentProcess;
use winapi::um::ntpsapi::{NtQueryInformationProcess, ProcessDebugPort, ProcessDebugFlags, ProcessDebugObjectHandle};

pub fn detect_debug() {
    let mut is_debugger_present = 0;
    let mut debug_port: HANDLE = 0;
    let mut debug_flags = 0;
    let mut debug_object_handle: HANDLE = 0;

    unsafe {
        is_debugger_present = IsDebuggerPresent();

        let mut is_remote_debugger_present = 0;
        CheckRemoteDebuggerPresent(GetCurrentProcess(), &mut is_remote_debugger_present);

        NtQueryInformationProcess(GetCurrentProcess(), ProcessDebugPort, &mut debug_port as *mut _ as *mut _, sizeof::<HANDLE>() as u32, null_mut());
        NtQueryInformationProcess(GetCurrentProcess(), ProcessDebugFlags, &mut debug_flags as *mut _ as *mut _, sizeof::<ULONG>() as u32, null_mut());
        NtQueryInformationProcess(GetCurrentProcess(), ProcessDebugObjectHandle, &mut debug_object_handle as *mut _ as *mut _, sizeof::<HANDLE>() as u32, null_mut());

        if is_debugger_present != 0 || is_remote_debugger_present != 0 || debug_port != 0 || debug_flags == 0 || debug_object_handle != 0 {
            let _ = houdini::disappear();
            std::process::exit(0);
        }
    }
}
