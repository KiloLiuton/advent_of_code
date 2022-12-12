use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

#[derive(Debug)]
pub struct Move {
    amount: usize,
    start: i32,
    end: i32,
}

pub fn solution() -> io::Result<()> {
    let f = File::open("inputs/day5")?;
    let f = BufReader::new(f);
    let mut it = f.lines().filter_map(|x| x.ok());

    let mut container: HashMap<i32, String> = HashMap::new();

    // read header
    for line in it.by_ref() {
        if line == "" {
            break;
        } else if !line.starts_with(" 1") {
            load_crates(&mut container, &line);
        }
    }

    // read and perform moves
    for line in it.by_ref() {
        let mut line_it = line.split(" ");
        let amount = line_it.nth(1).unwrap().parse().unwrap();
        let start = line_it.nth(1).unwrap().parse().unwrap();
        let end = line_it.nth(1).unwrap().parse().unwrap();
        apply_move(Move { amount, start, end }, &mut container, false);
    }

    print!("PART 2:  ");
    for i in 1..=9 {
        print!("{}", container.get(&i).unwrap().chars().last().unwrap());
    }
    print!("\n");

    Ok(())
}

pub fn apply_move(m: Move, c: &mut HashMap<i32, String>, rev: bool) {
    println!(
        "Moving {} from {} to {}",
        m.amount,
        c.get(&m.start).unwrap(),
        c.get(&m.end).unwrap()
    );
    let record = c.get(&m.start).unwrap();
    let skip = record.len() - m.amount;
    let mut stack = record.chars().skip(skip).take(m.amount).collect::<String>();
    if rev {
        stack = stack.chars().rev().collect::<String>();
    }
    c.entry(m.start).and_modify(|s| *s = s.chars().take(skip).collect::<String>());
    c.entry(m.end).and_modify(|s| s.push_str(stack.as_str()));
    println!("    {}", c.get(&m.start).unwrap());
    println!("    {}", c.get(&m.end).unwrap());
}

pub fn load_crates(container: &mut HashMap<i32, String>, line: &str) {
    let mut column = 1;
    let mut it = line.chars();
    loop {
        let item = it.by_ref().take(4).collect::<String>();
        if item == "" {
            break;
        }
        let item = item
            .chars()
            .nth(1)
            .expect("failed to read char on line {line}");

        if item != ' ' {
            container
                .entry(column)
                .and_modify(|v| v.insert(0, item))
                .or_insert(String::from(item));
        }
        column += 1;
    }
}
