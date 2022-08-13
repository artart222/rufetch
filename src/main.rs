use std::collections::HashMap;

mod config;
use crate::config::{read_config, Config};

mod get_info;
use crate::get_info::{cpu_count, get_mem, get_swap, os_info};

use sysinfo::{System, SystemExt};

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

    // Finding number of lines of ascii string.
    // And finding len of longest line.
    let mut longest_len = 0;
    let mut lines = 0;
    for line in config.ascii.to_string().lines() {
        lines += 1;
        if line.len() > longest_len {
            longest_len = line.len();
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
            for _ in line.len()..longest_len {
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
