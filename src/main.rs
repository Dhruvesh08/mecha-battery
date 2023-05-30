use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("/sys/class/power_supply/bq27441-0/uevent").unwrap();
    let reader = BufReader::new(file);
    let mut voltage_now = 0;
    let mut current_now = 0;
    let mut capacity = 0;
    let mut temp = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        if line.starts_with("POWER_SUPPLY_VOLTAGE_NOW=") {
            voltage_now = line[25..].parse().unwrap();
        } else if line.starts_with("POWER_SUPPLY_CURRENT_NOW=") {
            current_now = line[26..].parse().unwrap();
        } else if line.starts_with("POWER_SUPPLY_CAPACITY=") {
            capacity = line[22..].parse().unwrap();
        } else if line.starts_with("POWER_SUPPLY_TEMP=") {
            temp = line[18..].parse().unwrap();
        }
    }
    println!("voltage_now: {}", voltage_now);
    println!("current_now: {}", current_now);
    println!("capacity: {}", capacity);
    println!("temp: {}", temp);
}
