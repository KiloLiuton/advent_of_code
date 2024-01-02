use std::collections::{VecDeque, HashMap};

use common::read_lines;

fn day3_1() -> u32
{
    let lines = read_lines("inputs/in3.txt");
    let mut part_numbers: HashMap<(usize, usize), u32> = HashMap::new();
    for row in 0 .. lines.len()
    {
        let line = lines.iter().nth(row).unwrap();
        for (col, ch) in line.chars().enumerate()
        {
            if ch.is_digit(10) | (ch == '.')
            {
                continue
            }
            if (row > 0) & (col > 0)
            {
                let prev_line = lines.iter().nth(row - 1).unwrap();
                if let Some((pos, part_num)) = get_part_number(line, col - 1)
                {
                    part_numbers.insert((row, pos), part_num);
                }
                if let Some((pos, part_num)) = get_part_number(line, col + 1)
                {
                    part_numbers.insert((row, pos), part_num);
                }
            }
            else if row > 0
            {
            }
            else if col > 0
            {
            }
        }
    }
    666
}

fn get_part_number(line: &str, pos: usize) -> Option<(usize, u32)>
{
    let mut digits = VecDeque::new();
    // Add digits to the right, if any
    let mut offset = 0;
    let mut part_number_pos = pos;
    while let Some(c) = line.chars().nth(pos + offset)
    {
        println!("Analyzing right char {}", c);
        if let Some(d) = c.to_digit(10)
        {
            digits.push_back(d);
        }
        else
        {
            break
        }
        offset += 1;
    }
    if digits.len() == 0
    {
        return None
    }
    // Add digits to the left, if any
    offset = 1;
    if offset < pos
    {
        while let Some(c) = line.chars().nth(pos - offset)
        {
            println!("Analyzing left char {}", c);
            if let Some(d) = c.to_digit(10)
            {
                part_number_pos = pos - offset;
                digits.push_front(d);
            }
            else
            {
                break
            }
            offset += 1;
            if offset > pos
            {
                break
            }
        }
    }
    if digits.len() == 0
    {
        return None
    }
    let mut part_num = 0;
    for (i, d) in digits.iter().rev().enumerate()
    {
        part_num += d * 10_u32.pow(i as u32);
    }
    Some((part_number_pos, part_num))
}

fn main()
{
    println!("Answer part 1: {}",day3_1());
}
