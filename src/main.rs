mod day_one;
mod day_two;

fn main() {
    let day: u8 = 2;

    match day {
        1 => day_one::code::main(),
        2 => day_two::code::main(),
        _ => println!("No valid day!")
    }
}
