use std::env;

pub fn main() {
    let input = match env::var("DAY_ONE_INPUT") {
        Ok(seq) => seq.split(' ').map(String::from).collect::<Vec<String>>(),
        Err(_) => {
            eprintln!("DAY_ONE_INPUT environment variable not found, using default sequence");
            vec![
                "L68".to_string(),
                "L30".to_string(),
                "R48".to_string(),
                "L5".to_string(),
                "R60".to_string(),
                "L55".to_string(),
                "L1".to_string(),
                "L99".to_string(),
                "R14".to_string(),
                "L82".to_string()
            ]
        }
    };

    part_one(&input);
    part_two(&input);
}

fn part_one(sequence: &Vec<String>) {
    let mut current: i16 = 50;
    let mut counter: u16 = 0;

    for combination in sequence {
        let direction = combination.chars().next().unwrap();
        let number = combination[1..].parse::<i16>().unwrap();
        if direction == 'L' {
            for _ in 0..number {
                current -= 1;
                if current < 0 {
                    current = 99;
                }
            }
        } else if direction == 'R' {
            for _ in 0..number {
                current += 1;
                if current > 99 {
                    current = 0;
                }
            }
        } else {
            eprintln!("Error in {combination}! Skipping!");
        }
        if current == 0 {
            counter += 1;
        }
    }
    println!("Secret code one: {counter}")
}

fn part_two(sequence: &Vec<String>) {
    let mut current: i16 = 50;
    let mut counter: u16 = 0;

    for combination in sequence {
        let direction = combination.chars().next().unwrap();
        let number = combination[1..].parse::<i16>().unwrap();
        if direction == 'L' {
            for _ in 0..number {
                current -= 1;
                if current < 0 {
                    current = 99;
                }
                if current == 0 {
                    counter += 1;
                }
            }
        } else if direction == 'R' {
            for _ in 0..number {
                current += 1;
                if current > 99 {
                    current = 0;
                }
                if current == 0 {
                    counter += 1;
                }
            }
        } else {
            eprintln!("Error in {combination}! Skipping!");
        }
    }
    println!("Secret code two: {counter}")
}
