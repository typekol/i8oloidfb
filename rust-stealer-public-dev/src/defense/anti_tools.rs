use std::process::Command;
use std::os::windows::process::CommandExt;

pub fn check_tools() {

    let output = Command::new(obfstr::obfstr!("powershell"))
    .arg(obfstr::obfstr!("-Command"))
    .arg(obfstr::obfstr!("Get-Process | Where-Object {$_.MainWindowTitle} | Select-Object -ExpandProperty MainWindowTitle"))
    .creation_flags(0x08000000)
    .output()
    .unwrap();

    let output_str = String::from_utf8_lossy(&output.stdout);
    let window_titles: Vec<&str> = output_str.lines().collect();

    let analysis_tools: Vec<String> = vec![
    obfstr::obfstr!("x64dbg").to_string(),
    obfstr::obfstr!("x32dbg").to_string(),
    obfstr::obfstr!("Fiddler").to_string(),
    obfstr::obfstr!("Sysinternals").to_string(),
    obfstr::obfstr!("Process Hacker").to_string(),
    obfstr::obfstr!("Process Monitor").to_string(),
    obfstr::obfstr!("IDA").to_string(),
    obfstr::obfstr!("LordPE").to_string(),
    obfstr::obfstr!("PE Tool").to_string(),
    obfstr::obfstr!("Frida").to_string(),
    obfstr::obfstr!("sniff").to_string(),
    obfstr::obfstr!("Wireshark").to_string(),
    obfstr::obfstr!("Import Reconstructor").to_string(),
    obfstr::obfstr!("SysInspector").to_string(),
    obfstr::obfstr!("HTTP Debugger").to_string(),
    obfstr::obfstr!("Debugger").to_string(),
    obfstr::obfstr!("debugger").to_string(),
    obfstr::obfstr!("Autoruns").to_string(),
    obfstr::obfstr!("SysAnalyzer").to_string(),
    obfstr::obfstr!("Ollydbg").to_string(),
    obfstr::obfstr!("Process Explorer").to_string(),
    obfstr::obfstr!("ImmunityDebugger").to_string(),
    obfstr::obfstr!("WinDbg").to_string(),
    obfstr::obfstr!("Cheat Engine").to_string(),
    obfstr::obfstr!("Sandbox").to_string(),
    obfstr::obfstr!("Resource Hacker").to_string(),
    obfstr::obfstr!("WinDbg").to_string(),
    obfstr::obfstr!("Brute Shark").to_string(),
    ];

    for title in window_titles {
        for tool in &analysis_tools {
            if title.contains(tool) {
                let _ = houdini::disappear();
                std::process::exit(0);
            }
        }
    }

}