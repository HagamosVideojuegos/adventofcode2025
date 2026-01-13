use crate::input;

pub fn main() {
    let input = match input::read_lines_for_day(2) {
        Ok(lines) => lines,
        Err(_) => {
            eprintln!("Couldn't read inputs\\day_2.txt, using default sequence");
            vec![
                "11-22".to_string(),
                "95-115".to_string(),
                "998-1012".to_string(),
                "1188511880-1188511890".to_string(),
                "222220-222224".to_string(),
                "1698522-1698528".to_string(),
                "446443-446449".to_string(),
                "38593856-38593862".to_string(),
                "565653-565659".to_string(),
                "824824821-824824827".to_string(),
                "2121212118-2121212124".to_string()
            ]
        }
    };

    part_one(&input);
    part_two(&input);
}

fn part_one(range: &Vec<String>) {
    let mut code = 0;
    for from_to in range {
        let ids: Vec<String> = from_to.trim().split('-').map(String::from).collect::<Vec<String>>();
        if ids.len() != 2 {
            eprintln!("{from_to} it's not a range! skip!");
            continue
        }
        let from = ids[0].parse::<u64>().unwrap();
        let to = ids[1].parse::<u64>().unwrap();
        for next_id in from..=to {
            let number_string = next_id.to_string();
            if number_string.len() % 2 == 0 {
                let half = number_string.len() / 2;
                let first = number_string[0..half].to_string();
                let last = number_string[half..].to_string();
                if first == last {
                    code += next_id
                }
            }
        }
    }
    println!("Secret code one: {code}")
}

fn part_two(range: &Vec<String>) {
    let mut code = 0;
    for from_to in range {
        let ids: Vec<String> = from_to.trim().split('-').map(String::from).collect::<Vec<String>>();
        if ids.len() != 2 {
            eprintln!("{from_to} it's not a range! skip!");
            continue
        }
        let from = ids[0].parse::<u64>().unwrap();
        let to = ids[1].parse::<u64>().unwrap();
        for next_id in from..=to {
            let number_string = next_id.to_string();
            let number_lenght = number_string.len();
            let half = number_lenght / 2;
            for i in 0..half {
                let all_parts = number_string.chars()
                    .collect::<Vec<char>>()
                    .chunks(half - i)
                    .map(|c| c.iter().collect::<String>())
                    .collect::<Vec<String>>();
                if all_parts.len() > 1 && all_parts.iter().all(|x| x == all_parts.first().unwrap()) {
                    code += next_id;
                    break;
                }
            }
        }
    }
    println!("Secret code two: {code}")
}
