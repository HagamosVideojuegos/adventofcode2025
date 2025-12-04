mod day_one;
mod day_two;

fn main() {
    let day: u8 = 1;

    match day {
        1 => day_one::code::main(),
        _ => println!("No valid day!")
    }
}
