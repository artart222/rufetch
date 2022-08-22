use dirs;
use serde_derive::Deserialize;
use std::env::consts::OS;

#[derive(Deserialize)]
pub struct Config {
    pub ram_unit: char,
    pub swap_unit: char,
    pub free_mem: bool,
    pub total_mem: bool,
    pub used_mem: bool,
    pub free_swap: bool,
    pub total_swap: bool,
    pub used_swap: bool,
    pub os_name: bool,
    pub os_version: bool,
    pub kernel_version: bool,
    pub host_name: bool,
    pub user_name: bool,
    pub up_time: bool,
    pub cores_count: bool,
    pub cpu_brand: bool,
    pub cpu_frequency: bool,
    pub ascii: String,
}

pub fn read_config() -> Config {
    // Default config.
    let config = Config {
        ram_unit: 'K',
        swap_unit: 'K',
        free_mem: true,
        total_mem: true,
        used_mem: true,
        free_swap: true,
        total_swap: true,
        used_swap: true,
        os_name: true,
        os_version: true,
        kernel_version: true,
        host_name: true,
        user_name: true,
        cores_count: true,
        cpu_brand: true,
        cpu_frequency: true,
        up_time: true,
        ascii: "".to_string(),
    };

    // This variable for storing config file.
    let file: Result<String, std::io::Error>;
    let home_dir_addr = dirs::home_dir(); // Home directory address.
    if home_dir_addr.is_some() {
        // Trying to open and read config file in windows and other operating systems.
        if OS == "windows" {
            file = std::fs::read_to_string(format!(
                "{}{}",
                home_dir_addr.unwrap().display(),
                "\\AppData\\Local\\rufetch\\rufetch.toml"
            ));
        } else {
            file = std::fs::read_to_string(format!(
                "{}{}",
                home_dir_addr.unwrap().display(),
                "/.config/rufetch/rufetch.toml"
            ));
        }

        if file.is_ok() {
            match toml::from_str(&file.unwrap()) {
                Ok(value) => return value,
                Err(_) => {
                    eprintln!(
                        "The structure of your config file is not correct.\nUsing default config."
                    );
                    return config;
                }
            };
        } else {
            eprintln!("Can't open/read config file");
            return config;
        }
    } else {
        eprintln!("Can't find your home directory address.");
        return config;
    }
}
