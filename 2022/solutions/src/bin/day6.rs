use std::fs;
use std::io;
use itertools::Itertools;

pub fn main() -> io::Result<()> {
    let signal = fs::read_to_string("inputs/day6").expect("read file failed");
    let mut it = signal.chars();

    let mut sig = [' ', ' ', ' ', ' '];

    for i in 0..=3 {
        sig[i] = it.next().expect("Failed to read char!");
    }

    let mut result = 4;
    loop {
        if 4 == {
            let mut s = 0;
            for _ in sig.iter().unique() {
                s += 1;
            }
            s
        } {
            break;
        }
        for i in 0..=2 {
            sig[i] = sig[i+1];
        }
        sig[3] = it.next().expect("Failed to read char!");
        result += 1;
    }
    println!("PART 1: {result}");

    let mut it = signal.chars();

    let mut sig = [' '; 14];

    for i in 0..=13 {
        sig[i] = it.next().expect("Failed to read char!");
    }

    let mut result = 14;
    loop {
        if 14 == {
            let mut s = 0;
            for _ in sig.iter().unique() {
                s += 1;
            }
            s
        } {
            break;
        }
        for i in 0..=12 {
            sig[i] = sig[i+1];
        }
        sig[13] = it.next().expect("Failed to read char!");
        result += 1;
    }
    println!("PART 2: {result}");

    Ok(())
}
