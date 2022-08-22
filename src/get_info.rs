use crate::unit_converter;
use crate::Config;
use std::collections::HashMap;
use sysinfo::{ProcessorExt, System, SystemExt};
use whoami;

pub fn get_mem<'a>(sys: &'a System, config: &'a Config) -> HashMap<&'a str, u64> {
    return HashMap::from([
        (
            "TOTAL_MEM",
            unit_converter('K', config.ram_unit, sys.total_memory()),
        ),
        (
            "FREE_MEM",
            unit_converter('K', config.ram_unit, sys.free_memory()),
        ),
        (
            "USED_MEM",
            unit_converter('K', config.ram_unit, sys.used_memory()),
        ),
    ]);
}

pub fn get_swap<'a>(sys: &'a System, config: &'a Config) -> HashMap<&'a str, u64> {
    return HashMap::from([
        (
            "TOTAL_SWAP",
            unit_converter('K', config.swap_unit, sys.total_swap()),
        ),
        (
            "FREE_SWAP",
            unit_converter('K', config.swap_unit, sys.free_swap()),
        ),
        (
            "USED_SWAP",
            unit_converter('K', config.swap_unit, sys.used_swap()),
        ),
    ]);
}

pub fn os_info(sys: &System) -> HashMap<&str, String> {
    let mut info = std::collections::HashMap::new();

    let os_name = sys.name();
    if os_name.is_some() {
        info.insert("OS_NAME", os_name.unwrap());
    } else {
        info.insert("OS_NAME", "Not found".to_string());
    }

    let os_version = sys.os_version();
    if os_version.is_some() {
        info.insert("OS_VERSION", os_version.unwrap());
    } else {
        info.insert("OS_VERSION", "Not found".to_string());
    }

    let kernel_version = sys.kernel_version();
    if kernel_version.is_some() {
        info.insert("KERNEL_VERSION", kernel_version.unwrap());
    } else {
        info.insert("KERNEL_VERSION", "Not found".to_string());
    }

    info.insert("HOSTNAME", whoami::hostname());
    info.insert("USERNAME", whoami::username());

    info.insert("UP_TIME", sys.uptime().to_string());

    return info;
}

pub fn cpu_info(sys: &System) -> HashMap<&str, String> {
    let mut info = HashMap::new();

    info.insert("CORES_COUNT", sys.processors().len().to_string());
    let average_frequency: u64 = {
        let mut tmp: u64 = 0;
        for cpu in sys.processors().into_iter() {
            tmp += cpu.frequency();
        }
        tmp / sys.processors().len() as u64
    };
    info.insert("FREQUENCY", average_frequency.to_string());
    info.insert("CPU_BRAND", sys.processors()[0].brand().to_string());
    info
}
