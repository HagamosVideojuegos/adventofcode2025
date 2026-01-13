use crate::input;

pub fn main() {
    let input = input::read_lines_for_day(6).unwrap_or_else(|_| {
        eprintln!("Couldn't read inputs\\day_6.txt, using default sequence");
        vec![
            "123 328  51 64 ".to_string(),
            " 45 64  387 23 ".to_string(),
            "  6 98  215 314".to_string(),
            "*   +   *   +  ".to_string(),
        ]
    });

    let first = part_one(Homework::new_list(&input));
    println!("Secret code one: {}", first);
    let second = part_two(Homework::new_list(&input));
    println!("Secret code two: {}", second);
}

fn part_one(homeworks: Vec<Homework>) -> u64 {
    homeworks.iter()
        .map(|homework| homework.calculate())
        .sum()
}

fn part_two(homeworks: Vec<Homework>) -> u64 {
    let mut rtl_homeworks: Vec<Homework>  = vec![];
    for homework in &homeworks {
        let mut str_numbers = homework.numbers.clone();
        let max = homework.numbers.iter().max().unwrap().to_string().len();
        for str_number in &mut str_numbers {
            if str_number.len() < max {
                for _ in 0..(max - str_number.len()) {
                    str_number.push(' ');
                }
            }
        }
        let mut numbers = vec![];
        for i in 0..max {
            let mut str_number = str_numbers.iter().map(|x| x.chars().nth(i).unwrap()).collect::<String>();
            str_number = str_number.trim().to_string();
            if str_number.is_empty() { continue; }
            numbers.push(str_number)
        }
        rtl_homeworks.push(Homework::new(numbers, homework.calc));
    }
    rtl_homeworks.iter().map(|homework| homework.calculate()).sum()
}

struct Homework {
    numbers: Vec<String>,
    calc: char
}

impl Homework {
    fn new_list(input: &Vec<String>) -> Vec<Homework> {
        let mut homeworks = vec![];
        let calc_line: Vec<char> = input.last().unwrap().chars().collect();
        let number_lines = input.iter().take(input.len() - 1).collect::<Vec<&String>>();
        let mut first_char_index: Option<usize> = None;
        for i in 0..calc_line.len() {
            if calc_line[i] == ' ' {
                continue
            } else if first_char_index == None {
                first_char_index = Some(i);
            } else if first_char_index.unwrap() == 0 {
                let numbers = number_lines.iter()
                    .map(|x| x.chars()
                        .take(i - first_char_index.unwrap())
                        .collect::<String>())
                    .collect();
                homeworks.push(Homework::new(numbers, calc_line[first_char_index.unwrap()]));
                first_char_index = Some(i);
            } else {
                let numbers = number_lines.iter()
                    .map(|x| x.chars()
                        .skip(first_char_index.unwrap())
                        .take(i - first_char_index.unwrap())
                        .collect::<String>()
                    ).collect();
                homeworks.push(Homework::new(numbers, calc_line[first_char_index.unwrap()]));
                first_char_index = Some(i);
            }
        }
        let numbers = number_lines.iter()
            .map(|x| x.chars()
                .skip(first_char_index.unwrap())
                .collect::<String>()
            ).collect();
        homeworks.push(Homework::new(numbers, calc_line[first_char_index.unwrap()]));
        homeworks
    }

    fn new(numbers: Vec<String>, calc: char) -> Homework {
        Homework { numbers, calc }
    }

    fn calculate(&self) -> u64 {
        if self.calc == '*' {
            self.numbers.iter().map(|x| x.trim().parse::<u64>().unwrap()).product::<u64>()
        } else if self.calc == '+' {
            self.numbers.iter().map(|x| x.trim().parse::<u64>().unwrap()).sum::<u64>()
        } else {
            panic!("Invalid calc type! Skipping!");
        }
    }
}
