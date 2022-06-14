use serde_derive::Deserialize;
use std::collections::HashMap;
use sysinfo::{RefreshKind, System, SystemExt};

#[derive(Deserialize)]
struct Config {
    ram_unit: char,
    swap_unit: char,
    free_mem: bool,
    total_mem: bool,
    used_mem: bool,
    free_swap: bool,
    total_swap: bool,
    used_swap: bool,
    cpu_count: bool,
}

fn unit_converter(current_unit: char, convert_to: char, input: u64) -> u64 {
    let units = HashMap::from([
        ('B', 1.0),
        ('K', 1000.0),
        ('M', 1000000.0),
        ('G', 1000000000.0),
        ('T', 1000000000000.0),
    ]);
    return (input as f64 * (units.get(&current_unit).unwrap() / units.get(&convert_to).unwrap()))
        .round() as u64;
}

fn read_config() -> Config {
    let config = Config {
        ram_unit: 'K',
        swap_unit: 'K',
        free_mem: true,
        total_mem: true,
        used_mem: true,
        free_swap: true,
        total_swap: true,
        used_swap: true,
        cpu_count: true,
    };

    let file = std::fs::read_to_string("/home/artin/.config/rufetch/rufetch.toml");
    if file.is_ok() {
        match toml::from_str(&file.unwrap()) {
            Ok(value) => return value,
            Err(_) => {
                println!("The structure of your config file is not correct.");
                println!("Using default config.");
                return config;
            }
        };
    } else {
        println!("Can't open/read config file");
        return config;
    }
}

fn get_mem(sys: &System, config: &Config) -> (u64, u64, u64) {
    let mut total_mem: u64 = 0;
    if config.total_mem {
        total_mem = sys.total_memory();
    }
    let mut free_mem: u64 = 0;
    if config.free_mem {
        free_mem = sys.free_memory();
    }
    let mut used_mem: u64 = 0;
    if config.used_mem {
        used_mem = sys.used_memory();
    }
    return (
        unit_converter('K', config.ram_unit, total_mem),
        unit_converter('K', config.ram_unit, free_mem),
        unit_converter('K', config.ram_unit, used_mem),
    );
}

fn get_swap(sys: &System, config: &Config) -> (u64, u64, u64) {
    let mut total_swap: u64 = 0;
    if config.total_swap {
        total_swap = sys.total_swap();
    }
    let mut free_swap: u64 = 0;
    if config.free_swap {
        free_swap = sys.free_swap();
    }
    let mut used_swap: u64 = 0;
    if config.used_swap {
        used_swap = sys.used_swap();
    }
    return (
        unit_converter('K', config.swap_unit, total_swap),
        unit_converter('K', config.swap_unit, free_swap),
        unit_converter('K', config.swap_unit, used_swap),
    );
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
    let config = read_config();
    println!("{:.2}", get_mem(&sys, &config).0);
    println!("{:.2}", get_mem(&sys, &config).1);
    println!("{:.2}", get_mem(&sys, &config).2);
}
