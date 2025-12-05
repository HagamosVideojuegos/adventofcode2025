mod day_one;
mod day_two;
mod day_three;

fn main() {
    let day: u8 = 3;

    match day {
        1 => day_one::code::main(),
        2 => day_two::code::main(),
        3 => day_three::code::main(),
        _ => println!("No valid day!")
    }
}
