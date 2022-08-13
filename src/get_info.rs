use crate::unit_converter;
use crate::Config;
use sysinfo::{RefreshKind, System, SystemExt};

pub fn get_mem(sys: &System, config: &Config) -> (u64, u64, u64) {
    return (
        unit_converter('K', config.ram_unit, sys.total_memory()),
        unit_converter('K', config.ram_unit, sys.free_memory()),
        unit_converter('K', config.ram_unit, sys.used_memory()),
    );
}

pub fn get_swap(sys: &System, config: &Config) -> (u64, u64, u64) {
    return (
        unit_converter('K', config.swap_unit, sys.total_swap()),
        unit_converter('K', config.swap_unit, sys.free_swap()),
        unit_converter('K', config.swap_unit, sys.used_swap()),
    );
}

pub fn os_info(sys: &System) -> (String, String, String, String) {
    let mut info: (String, String, String, String) = (
        "NONE".to_string(),
        "NONE".to_string(),
        "NONE".to_string(),
        "NONE".to_string(),
    );

    let os_name = sys.name();
    if os_name.is_some() {
        info.0 = os_name.unwrap();
    }

    let os_version = sys.os_version();
    if os_version.is_some() {
        info.1 = os_version.unwrap()
    }

    let kernel_version = sys.kernel_version();
    if kernel_version.is_some() {
        info.2 = kernel_version.unwrap()
    }

    let host_name = sys.host_name();
    if host_name.is_some() {
        info.3 = host_name.unwrap()
    }

    return info;
}

pub fn cpu_count() -> u8 {
    let s = System::new_with_specifics(RefreshKind::new().with_cpu());

    let mut processors_count: u8 = 0;
    for _ in s.processors() {
        processors_count += 1;
    }

    return processors_count;
}
