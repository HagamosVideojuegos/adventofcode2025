use crate::input;

pub fn main() {
    let input = input::read_lines_for_day(3).unwrap_or_else(|_| {
        eprintln!("Couldn't read inputs\\day_3.txt, using default sequence");
        vec![
            "987654321111111".to_string(),
            "811111111111119".to_string(),
            "234234234234278".to_string(),
            "818181911112111".to_string()
        ]
    });

    part_one(&input);
    part_two(&input);
}

fn part_one(range: &Vec<String>) {
    let mut code: u32 = 0;
    for battery in range {
        if battery.len() < 2 {
            eprintln!("{battery} it's too short! skip!");
            continue
        }
        if battery.chars().all(|c| c.is_numeric() == false) {
            eprintln!("{battery} it's not a number! skip!");
            continue
        }
        let battery_code = recursive_max_value(battery.to_string(), String::from(""), 2);
        code += battery_code.parse::<u32>().unwrap()
    }
    println!("Secret code one: {code}")
}

fn part_two(range: &Vec<String>) {
    let mut code: u128 = 0;
    for battery in range {
        if battery.len() < 12 {
            eprintln!("{battery} it's too short! skip!");
            continue
        }
        if battery.chars().all(|c| c.is_numeric() == false) {
            eprintln!("{battery} it's not a number! skip!");
            continue
        }
        let battery_code = recursive_max_value(battery.to_string(), String::from(""), 12);
        code += battery_code.parse::<u128>().unwrap()
    }
    println!("Secret code two: {code}")
}

fn recursive_max_value(battery: String, current_string: String, size: usize) -> String {
    let mut max_number = 0;
    let mut max_index = 0;
    for i in 0..battery.len() - (size - 1) {
        let current = battery.chars().nth(i).unwrap().to_string().parse::<u8>().unwrap();
        if max_number < current {
            max_number = current.to_string().parse::<u8>().unwrap();
            max_index = i
        }
    }
    if size > 1 {
        recursive_max_value(battery[max_index + 1..].to_string(), current_string.to_owned() + &max_number.to_string(), size - 1)
    } else {
        current_string.to_owned() + &max_number.to_string()
    }
}
