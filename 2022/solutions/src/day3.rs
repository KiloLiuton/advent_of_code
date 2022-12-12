use std::fs;

pub fn priority(letter: char) -> i32 {
    match letter {
        'a' => 1,
        'b' => 2,
        'c' => 3,
        'd' => 4,
        'e' => 5,
        'f' => 6,
        'g' => 7,
        'h' => 8,
        'i' => 9,
        'j' => 10,
        'k' => 11,
        'l' => 12,
        'm' => 13,
        'n' => 14,
        'o' => 15,
        'p' => 16,
        'q' => 17,
        'r' => 18,
        's' => 19,
        't' => 20,
        'u' => 21,
        'v' => 22,
        'w' => 23,
        'x' => 24,
        'y' => 25,
        'z' => 26,
        'A' => 27,
        'B' => 28,
        'C' => 29,
        'D' => 30,
        'E' => 31,
        'F' => 32,
        'G' => 33,
        'H' => 34,
        'I' => 35,
        'J' => 36,
        'K' => 37,
        'L' => 38,
        'M' => 39,
        'N' => 40,
        'O' => 41,
        'P' => 42,
        'Q' => 43,
        'R' => 44,
        'S' => 45,
        'T' => 46,
        'U' => 47,
        'V' => 48,
        'W' => 49,
        'X' => 50,
        'Y' => 51,
        'Z' => 52,
        _ => panic!("letter not in alphabet!"),
    }
}

pub fn solution() -> Result<(), String> {
    let rucksacks: Vec<String> = fs::read_to_string("inputs/day3")
        .expect("Can't read file!")
        .split_terminator("\n")
        .map(|line| String::from(line.trim()))
        .collect();

    let mut sum_priorities = 0;
    for sack in &rucksacks {
        let n = sack.len() / 2;
        'bag: for i in 0..n {
            for j in n..sack.len() {
                let left = sack.chars().nth(i).unwrap();
                let right = sack.chars().nth(j).unwrap();
                if left == right {
                    sum_priorities += priority(left);
                    break 'bag;
                }
            }
        }
    }

    println!("PART 1:\nSum of priorities is {}", sum_priorities);

    sum_priorities = 0;
    let num_groups = rucksacks.len() / 3;
    for i in 0..num_groups {
        let mut common_items = String::new();

        let sack1 = &rucksacks[i * 3];
        let sack2 = &rucksacks[i * 3 + 1];
        let sack3 = &rucksacks[i * 3 + 2];

        for item in sack2.chars() {
            if sack1.contains(item) {
                common_items.push(item);
            }
        }
        for item in sack3.chars() {
            if common_items.contains(item) {
                sum_priorities += priority(item);
                break;
            }
        }
    }

    println!("PART 2:\nSum of priorities is {}", sum_priorities);

    Ok(())
}
