use solutions::get_nth_word;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::iter::Peekable;

pub fn main() -> io::Result<()> {
    let f = BufReader::new(File::open("inputs/day7")?);
    let mut it = f.lines().filter_map(|x| x.ok()).peekable();

    let mut curr_path: Vec<String> = Vec::new();
    let mut all_dirs: HashMap<String, usize> = HashMap::new();

    while let Some(line) = it.next() {
        if !line.starts_with("$") {
            panic!("Main loop is supposed to get only commands!!!");
        }

        let cmd = get_nth_word(&line, 2).expect("Cannot parse command on line {line}!");
        match cmd {
            "cd" => {
                let arg = get_nth_word(&line, 3).expect("Cannot parse arg in line {line}!");
                match arg {
                    ".." => {
                        let _ = curr_path.pop().expect("Could not pop dir from curr_path!");
                    }
                    _ => {
                        curr_path.push(arg.to_string());
                        all_dirs.entry(curr_path.join("/")).or_insert(0);
                    }
                }
                println!("CURR PATH: {:?}", curr_path);
            }
            "ls" => {
                let dirsize = expand_dir(&mut it);
                for i in 0..curr_path.len() {
                    let key = &curr_path[0..=i].join("/");
                    *all_dirs.entry(key.clone()).or_insert(0) += dirsize;
                }
            }
            _ => panic!("Unrecognized command on line {line}!"),
        };
    }

    let mut sum = 0;
    for (_, size) in &all_dirs {
        let size = *size;
        if size <= 100_000 {
            sum += size;
        }
    }
    println!("PART 1: {}", sum);

    let min_dir_size: usize = 8_381_165;

    let x = *all_dirs
        .values()
        .filter(|x| x >= &&min_dir_size)
        .min()
        .unwrap();

    for (dir, size) in &all_dirs {
        if size >= &min_dir_size {
            println!("{dir}: {size}");
        }
    }

    println!("PART 2: {x}");

    Ok(())
}

pub fn expand_dir<I: Iterator<Item = String>>(it: &mut Peekable<I>) -> usize {
    let mut result: usize = 0;
    while let Some(line) = it.peek() {
        // if the next line is a shell command, stop
        if line.starts_with("$") {
            break;
            // ignore lines that list other directories, they will be accounted for on cd command
        }
        if let Some(size) = get_nth_word(line, 1)
            .expect("Cannot read prefix on line {line}")
            .parse::<usize>()
            .ok()
        {
            result += size;
        }
        println!("    {line}");
        it.next();
    }
    println!("    {result}");
    result
}
