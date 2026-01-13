use crate::input;

pub fn main() {
    let input = match input::read_lines_for_day(5) {
        Ok(lines) => lines,
        Err(_) => {
            eprintln!("Couldn't read inputs\\day_5.txt, using default sequence");
            vec![
                "3-5".to_string(),
                "10-14".to_string(),
                "16-20".to_string(),
                "12-18".to_string(),
                "".to_string(),
                "1".to_string(),
                "5".to_string(),
                "8".to_string(),
                "11".to_string(),
                "17".to_string(),
                "32".to_string()
            ]
        }
    };

    let first = part_one(Ingredients::new(&input));
    println!("Secret code one: {}", first);
    let second = part_two(Ingredients::new(&input));
    println!("Secret code two: {}", second);
}

fn part_one(ingredients: Ingredients) -> u64 {
    let list = ingredients.fresh_availables();
    list.len() as u64
}

fn part_two(ingredients: Ingredients) -> u64 {
    ingredients.fresh_count()
}

struct Ingredients {
    fresh: Vec<String>,
    available: Vec<String>
}

impl Ingredients {
    fn new(input: &Vec<String>) -> Ingredients {
        let mut fresh = vec![];
        input.iter()
            .take_while(|x| x.len() > 0)
            .for_each(|x| fresh.push(x.to_string()));
        let mut available = vec![];
        input.iter()
            .skip_while(|x| x.len() > 0)
            .skip(1)
            .for_each(|x| available.push(x.to_string()));
        Ingredients { fresh, available }
    }

    fn fresh_availables(&self) -> Vec<String> {
        self.available.iter().filter(|available| {
            self.fresh.iter().any(|fresh| {
                let parts = fresh.split('-').collect::<Vec<&str>>();
                if parts.len() != 2 {
                    eprintln!("Invalid input: {}", fresh);
                    return false;
                }
                let start = parts[0].parse::<u64>().unwrap();
                let end = parts[1].parse::<u64>().unwrap();
                let available_num = available.parse::<u64>().unwrap();
                available_num >= start && available_num <= end
            })
        }).cloned().collect()
    }

    fn fresh_count(&self) -> u64 {
        let mut counter: u64 = 0;
        let mut list: Vec<String> = vec![];
        let mut sorted_fresh = self.fresh.clone();
        sorted_fresh.sort_by(|a, b| {
            let a_parts = a.split('-').collect::<Vec<&str>>();
            let a_start = a_parts[0].parse::<u64>().unwrap();
            let b_parts = b.split('-').collect::<Vec<&str>>();
            let b_start = b_parts[0].parse::<u64>().unwrap();
            a_start.cmp(&b_start)
        });
        for i in 0..sorted_fresh.len() {
            let current_parts = sorted_fresh[i].split('-').collect::<Vec<&str>>();
            let current_start = current_parts[0].parse::<u64>().unwrap();
            let current_end = current_parts[1].parse::<u64>().unwrap();
            if i > 0 {
                let previous_parts = list[list.len() - 1].split('-').collect::<Vec<&str>>();
                let previous_start = previous_parts[0].parse::<u64>().unwrap();
                let previous_end = previous_parts[1].parse::<u64>().unwrap();
                if previous_end >= current_start {
                    list.remove(list.len() - 1);
                    let max_end = previous_end.max(current_end);
                    list.push(format!("{}-{}", previous_start, max_end));
                } else {
                    list.push(sorted_fresh[i].to_string());
                }
            } else {
                list.push(sorted_fresh[i].to_string());
            }
        }
        for range in list.iter() {
            let current_parts = range.split('-').collect::<Vec<&str>>();
            let current_start = current_parts[0].parse::<u64>().unwrap();
            let current_end = current_parts[1].parse::<u64>().unwrap();
            counter += current_end - current_start + 1;
        }
        counter
    }
}
