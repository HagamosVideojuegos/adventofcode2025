use crate::input;
use crate::day_seven::tachyon::Tachyon;

pub fn main() {
    let input = input::read_lines_for_day(7).unwrap_or_else(|_| {
        eprintln!("Couldn't read inputs\\day_7.txt, using default sequence");
        vec![
            ".......S.......".to_string(),
            "...............".to_string(),
            ".......^.......".to_string(),
            "...............".to_string(),
            "......^.^......".to_string(),
            "...............".to_string(),
            ".....^.^.^.....".to_string(),
            "...............".to_string(),
            "....^.^...^....".to_string(),
            "...............".to_string(),
            "...^.^...^.^...".to_string(),
            "...............".to_string(),
            "..^...^.....^..".to_string(),
            "...............".to_string(),
            ".^.^.^.^.^...^.".to_string(),
            "...............".to_string()
        ]
    });

    let mut tachyon = Tachyon::new(input);
    tachyon.shoot_beam();

    part_one(&tachyon);
    part_two(&mut tachyon);
}

fn part_one(tachyon: &Tachyon) {
    let counter = tachyon.count_splits();
    println!("Secret code one: {counter}")
}

fn part_two(tachyon: &mut Tachyon) {
    let counter = tachyon.count_quantum_splits();
    println!("Secret code two: {counter}")
}