mod day_one;
mod day_two;
mod day_three;
mod day_four;
mod day_five;
mod day_six;
mod day_seven;
mod input;

fn main() {
    let day: u8 = 7;

    match day {
        1 => day_one::code::main(),
        2 => day_two::code::main(),
        3 => day_three::code::main(),
        4 => day_four::code::main(),
        5 => day_five::code::main(),
        6 => day_six::code::main(),
        7 => day_seven::code::main(),
        _ => println!("No valid day!")
    }
}
