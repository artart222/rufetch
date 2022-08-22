// For reading and storing config.
mod config;
use crate::config::{read_config, Config};
// Some functions for getting infos.
mod get_info;
use crate::get_info::{cpu_info, get_mem, get_swap, os_info};
use sysinfo::{System, SystemExt};

fn unit_converter(current_unit: char, convert_to: char, input: u64) -> u64 {
    let units = std::collections::HashMap::from([
        ('B', 1.0),
        ('K', 1000.0),
        ('M', 1000000.0),
        ('G', 1000000000.0),
        ('T', 1000000000000.0),
    ]);
    return (input as f64 * (units.get(&current_unit).unwrap() / units.get(&convert_to).unwrap()))
        .round() as u64;
}

fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();
    let config = read_config();
    let mut datas: Vec<String> = Vec::new();
    if config.total_mem || config.used_mem || config.free_mem {
        let mem_info = get_mem(&sys, &config);
        if config.total_mem {
            datas.push(" Total memory: ".to_string() + &mem_info["TOTAL_MEM"].to_string());
        }
        if config.free_mem {
            datas.push(" Free memory: ".to_string() + &mem_info["FREE_MEM"].to_string());
        }
        if config.used_mem {
            datas.push(" Used memory: ".to_string() + &mem_info["USED_MEM"].to_string());
        }
    }
    if config.total_swap || config.used_swap || config.free_swap {
        let swap_info = get_swap(&sys, &config);
        if config.total_swap {
            datas.push(" Total swap: ".to_string() + &swap_info["TOTAL_SWAP"].to_string());
        }
        if config.free_swap {
            datas.push(" Free swap: ".to_string() + &swap_info["FREE_SWAP"].to_string());
        }
        if config.used_swap {
            datas.push(" Used swap: ".to_string() + &swap_info["USED_SWAP"].to_string());
        }
    }

    if config.os_name
        || config.os_version
        || config.kernel_version
        || config.host_name
        || config.user_name
        || config.up_time
    {
        let os_info = os_info(&sys);
        if config.os_name {
            datas.push(" Operating system: ".to_string() + &os_info["OS_NAME"].to_string());
        }
        if config.os_version {
            datas.push(" OS version: ".to_string() + &os_info["OS_VERSION"].to_string());
        }
        if config.kernel_version {
            datas.push(" Kernel version: ".to_string() + &os_info["KERNEL_VERSION"].to_string());
        }
        if config.host_name {
            datas.push(" Host: ".to_string() + &os_info["HOSTNAME"].to_string());
        }
        if config.user_name {
            datas.push(" User: ".to_string() + &os_info["USERNAME"].to_string());
        }
        if config.up_time {
            datas.push(" Uptime: ".to_string() + &os_info["UP_TIME"].to_string());
        }
    }

    if config.cores_count || config.cpu_frequency || config.cpu_brand {
        let cpu_info = cpu_info(&sys);
        datas.push(" Cpu brand: ".to_string() + &cpu_info["CPU_BRAND"].to_string());
        datas.push(" Cpu frequency: ".to_string() + &cpu_info["FREQUENCY"].to_string());
        datas.push(" Cpu cores: ".to_string() + &cpu_info["CORES_COUNT"].to_string());
    }

    // Finding number of lines of ascii string.
    // And finding len of longest line.
    let mut longest_len = 0;
    let mut lines = 0;
    for line in config.ascii.to_string().lines() {
        lines += 1;
        if line.chars().count() > longest_len {
            longest_len = line.chars().count();
        }
    }

    // Choose len of between ascii in datas in printing.
    let gap = {
        if lines == 0 {
            0
        } else {
            3
        }
    };

    // Making actual string of gap.
    let gap = {
        let mut white_space: String = "".to_string();
        for _ in 0..gap {
            white_space += " ";
        }
        white_space
    };

    let mut index = 0;
    // We will use this string later to add needed
    // white space to lines that are shorter than longest line
    // To make all lines len similar.
    let mut white_space: String = String::new();
    // If number of lines is more/equal to number of datas.
    if lines >= datas.len() {
        // Iterating over lines of ascii.
        for line in config.ascii.to_string().lines() {
            // Making white_space string.
            white_space = "".to_string(); // Reseting white_space.
            for _ in line.chars().count()..longest_len {
                white_space += " ";
            }
            // If we have data for printing this line will print that.
            if datas.len() >= index + 1 {
                println!("{}{}{}{}", line, white_space, gap, datas[index]);
            } else {
                // Printing just string without any data because we don't have any data!.
                println!("{}{}{}", line, white_space, gap);
            }
            index += 1;
        }
    } else {
        // Iterate over datas (I used index variable to work with indexes)
        // index is defined before we start process of printing.
        for _ in &datas {
            // Reseting white_space.
            white_space = "".to_string();
            // If we have data for printing this code block will print it.
            //Check this part
            if lines >= index + 1 {
                // Making white_space string.
                for _ in config.ascii.to_string().lines().nth(index).unwrap().len()..longest_len {
                    white_space += " ";
                }
                println!(
                    "{}{}{}{}",
                    config.ascii.to_string().lines().nth(index).unwrap(),
                    white_space,
                    gap,
                    datas[index]
                );
            } else {
                // Making white_space string.
                for _ in 0..longest_len {
                    white_space += " ";
                }
                // Printing just datas without any ascii anymore!.
                println!("{}{}{}", white_space, gap, datas[index]);
            }
            index += 1;
        }
    }
}
