use std::fs;
use std::io;

pub fn solution() -> io::Result<()> {
    let mut signal = fs::read_to_string("inputs/day6").expect("read file failed");

    let mut sig: [char; 3];
    let mut last_ch: char;

    let mut count = 0;
    for ch in signal.chars() {
        count += 1;
    }

    Ok(())
}
