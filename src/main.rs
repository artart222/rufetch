use serde_derive::Deserialize;
use std::{clone, collections::HashMap};
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
    ascii: String,
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
        ascii: "".to_string(),
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
    return (
        unit_converter('K', config.ram_unit, sys.total_memory()),
        unit_converter('K', config.ram_unit, sys.free_memory()),
        unit_converter('K', config.ram_unit, sys.used_memory()),
    );
}

fn get_swap(sys: &System, config: &Config) -> (u64, u64, u64) {
    return (
        unit_converter('K', config.swap_unit, sys.total_swap()),
        unit_converter('K', config.swap_unit, sys.free_swap()),
        unit_converter('K', config.swap_unit, sys.used_swap()),
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
    let mut datas: Vec<String> = Vec::new();
    if config.total_mem || config.used_mem || config.free_mem {
        let mem_info = get_mem(&sys, &config);
        if config.total_mem {
            datas.push(mem_info.0.to_string());
        }
        if config.free_mem {
            datas.push(mem_info.1.to_string());
        }
        if config.used_mem {
            datas.push(mem_info.2.to_string());
        }
    }
    if config.total_swap || config.used_swap || config.free_swap {
        let swap_info = get_swap(&sys, &config);
        if config.total_swap {
            datas.push(swap_info.0.to_string());
        }
        if config.free_swap {
            datas.push(swap_info.1.to_string());
        }
        if config.used_swap {
            datas.push(swap_info.2.to_string());
        }
    }

    let mut most_len = 0;
    let mut lines = 0;
    for line in config.ascii.to_string().lines() {
        lines += 1;
        if line.len() > most_len {
            most_len = line.len();
        }
    }

    let a = datas.len();
    let mut index = 0;
    if lines >= datas.len() {
        for line in config.ascii.to_string().lines() {
            let mut white_space: String = "".to_string();
            for i in line.len()..most_len {
                white_space += " ";
            }
            if datas.len() >= index + 1 {
                println!("{}{}   {}", line, white_space, datas[index]);
            } else {
                println!("{}{}   ", line, white_space);
            }
            index += 1;
        }
    } else {
        if lines == 0 {
            for data in datas {
                println!("{}", data);
            }
        } else {
            let mut i = 0;
            for data in datas {
                if i <= lines - 1 {
                    let mut white_space: String = "".to_string();
                    if config.ascii.to_string().lines().nth(i).unwrap().len() < most_len {
                        for i in
                            config.ascii.to_string().lines().nth(i + 1).unwrap().len()..most_len
                        {
                            white_space += " ";
                        }
                    }
                    println!(
                        "{}{}   {}",
                        config.ascii.to_string().lines().nth(i).unwrap(),
                        white_space,
                        data
                    );
                } else {
                    let mut white_space: String = "".to_string();
                    for i in 0..most_len {
                        white_space += " ";
                    }

                    println!("{}   {}", white_space, data);
                }
                i += 1;
            }
        }
    } // TODO: I definitely must fix this bullshit.
}
