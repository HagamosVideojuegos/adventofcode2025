use std::fs;
use std::io;

pub fn read_lines_for_day(day: u8) -> io::Result<Vec<String>> {
    let path = format!("inputs\\day_{}.txt", day);
    let content = fs::read_to_string(&path)?;
    Ok(content.lines().map(|s| s.to_string()).collect())
}
