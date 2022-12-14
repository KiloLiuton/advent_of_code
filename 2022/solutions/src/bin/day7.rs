// use std::collections::HashMap;
use solutions::get_nth_word;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

pub fn main() -> io::Result<()> {
    let f = BufReader::new(File::open("inputs/day7")?);
    let it = f.lines().filter_map(|x| x.ok());

    // let mut dirs: HashMap<String, usize> = HashMap::new();

    for line in it {
        if line.starts_with("$") {
            let cmd = get_nth_word(&line, 2).expect("Could not read cmd at line {line}!");
            match cmd {
                "cd" => println!("CD command found!"),
                "ls" => println!("LS command found!"),
                _ => panic!("Unrecognized cmd {cmd}!!"),
            }
        }
    }

    Ok(())
}
