use std::io::{self, BufRead};

fn main() {
    let mut part_1_fuel = 0;
    let mut part_2_fuel = 0;
    for line in io::stdin().lock().lines() {
        if let Ok(mass) = line.unwrap().parse::<u32>() {
            let mut fuel = get_fuel(mass);
            part_1_fuel += fuel;

            while fuel > 0 {
                part_2_fuel += fuel;
                fuel = get_fuel(fuel);
            }
        }
    }

    println!("Part 1: {}", part_1_fuel);
    println!("Part 2: {}", part_2_fuel);
}

fn get_fuel(mass: u32) -> u32 {
    match (mass / 3).checked_sub(2) {
        Some(ret) => ret,
        None => 0,
    }
}

#[test]
fn get_fuel_works() {
    assert_eq!(get_fuel(12), 2);
    assert_eq!(get_fuel(14), 2);
    assert_eq!(get_fuel(1969), 654);
    assert_eq!(get_fuel(100756), 33583);
}
