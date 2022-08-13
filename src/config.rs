use serde_derive::Deserialize;

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
    pub cpu_count: bool,
    pub ascii: String,
}

pub fn read_config() -> Config {
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
