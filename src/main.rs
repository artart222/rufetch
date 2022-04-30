use serde_derive::Deserialize;
use sysinfo::{RefreshKind, System, SystemExt};

#[derive(Deserialize)]
struct Config {
    ram_unit: char,
    swap_unit: char,
}

fn read_config() -> Config {
    let config = Config {
        ram_unit: 'M',
        swap_unit: 'M',
    };

    let file = std::fs::read_to_string("/home/artin/.config/rufetch/rufetch.toml");
    if file.is_ok() {
        let file = file.unwrap();
        let config_file = toml::from_str(&file);
        if config_file.is_ok() {
            return config_file.unwrap();
        }
    }

    return config;
}

fn get_mem(sys: &System, unit: char) -> (u64, u64, u64) {
    let total_mem_size = sys.total_memory();
    let free_mem_size = sys.free_memory();
    let used_mem_size = sys.used_memory();
    match unit {
        'B' => {
            return (
                total_mem_size * 1000,
                free_mem_size * 1000,
                used_mem_size * 1000,
            )
        }
        'K' => return (total_mem_size, free_mem_size, used_mem_size),
        'M' => {
            return (
                total_mem_size / 1000,
                free_mem_size / 1000,
                used_mem_size / 1000,
            )
        }
        'G' => {
            return (
                total_mem_size / 1000000,
                free_mem_size / 1000000,
                used_mem_size / 1000000,
            )
        }
        'T' => {
            return (
                total_mem_size / 1000000000,
                free_mem_size / 1000000000,
                used_mem_size / 1000000000,
            )
        }
        _ => return (0, 0, 0),
    }
}

fn get_swp(sys: &System, unit: char) -> (u64, u64, u64) {
    let total_swp_size = sys.total_swap();
    let free_swp_size = sys.free_swap();
    let used_swp_size = sys.used_swap();
    match unit {
        'B' => {
            return (
                total_swp_size * 1000,
                free_swp_size * 1000,
                used_swp_size * 1000,
            )
        }
        'K' => return (total_swp_size, free_swp_size, used_swp_size),
        'M' => {
            return (
                total_swp_size / 1000,
                free_swp_size / 1000,
                used_swp_size / 1000,
            )
        }
        'G' => {
            return (
                total_swp_size / 1000000,
                free_swp_size / 1000000,
                used_swp_size / 1000000,
            )
        }
        'T' => {
            return (
                total_swp_size / 1000000000,
                free_swp_size / 1000000000,
                used_swp_size / 1000000000,
            )
        }
        _ => return (0, 0, 0),
    }
}

fn os_info(sys: &System) -> (String, String, String, String) {
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

fn cpu_count() -> u8 {
    let s = System::new_with_specifics(RefreshKind::new().with_cpu());

    let mut processors_count: u8 = 0;
    for _ in s.processors() {
        processors_count += 1;
    }

    return processors_count;
}

fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();
    println!("{}/{}", get_mem(&sys, 'M').0, get_mem(&sys, 'M').2);
    println!("{}", os_info(&sys).0);

    println!("{}", cpu_count());
    println!("{}", read_config().ram_unit);
    println!("{}", get_mem(&sys, read_config().ram_unit).0);
}
