use common::read_lines;


fn day1_1() -> u32 {
    // let lines = read_lines("inputs/in1.txt");
    // let mut result: Vec<u32> = Vec::new();
    // for line in lines
    // {
    //     let mut digits = Vec::new();
    //     for ch in line.chars()
    //     {
    //         if let Some(digit) = ch.to_digit(10)
    //         {
    //             digits.push(digit);
    //         }
    //     }
    //     let d1 = digits[0];
    //     let d2 = digits[digits.len() - 1];
    //     result.push(10*d1 + d2);
    // }
    // result.iter().sum()

    read_lines("inputs/in1.txt")
        .iter()
        .filter_map(|line| {
            let digits: Vec<_> = line.chars().filter_map(|ch| ch.to_digit(10)).collect();
            let a = digits.iter().next()?;
            let b = digits.iter().last()?;
            Some (a * 10 + b)
        })
        .sum()
}

fn day1_22() -> u32 {
    let lines = read_lines("inputs/in1.txt");
    for line in lines {
        let mut i = 0;
        let mut digits = vec![];
        while i < line.len() {
            let d = match line.chars().nth(i).map(|ch| ch.to_digit(10)) {
                Some(x) => {
                    i += 1;
                    x
                },
                None =>
                    if line[i..].starts_with("one") {
                        i += 2;
                        1
                    } else if line[i..].starts_with("two") {
                        i += 3;
                        2
                    } else if line[i..].starts_with("three") {
                        i += 4;
                        3
                    } else if line[i..].starts_with("four") {
                        i += 4;
                        4
                    } else if line[i..].starts_with("five") {
                        i += 3;
                        5
                    } else if line[i..].starts_with("six") {
                        i += 3;
                        6
                    } else if line[i..].starts_with("seven") {
                        i += 4;
                        7
                    } else if line[i..].starts_with("eight") {
                        i += 4;
                        8
                    } else if line[i..].starts_with("nine") {
    
                        f
                        i += 3;
                        9
                    }
            };
            digits.push(d);
        }
        let a = digits.iter().next()?;
        let b = digits.iter().next()?;
        ans += a * 10 + b;
    }
    ans
}

fn day1_2() -> u32 {
    let lines = read_lines("inputs/in1.txt");
    let mut result: Vec<u32> = Vec::new();
    for line in lines
    {
        let mut digits = Vec::new();
        let mut positions = Vec::new();
        let mut pos = 0;
        for ch in line.chars()
        {
            if let Some(digit) = ch.to_digit(10)
            {
                digits.push(digit);
                positions.push(pos);
            }
            pos = pos + 1;
        }

        for (i, substring) in ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"].iter().enumerate()
        {
            if let Some(position) = line.find(substring)
            {
                digits.push((i + 1) as u32);
                positions.push(position);
            }
            if let Some(position) = line.rfind(substring)
            {
                digits.push((i + 1) as u32);
                positions.push(position);
            }
        }
        let mut d = (0, 0);
        if let Some((min_index, _)) = positions.iter().enumerate().min_by_key(|&(_, pos)| pos)
        {
            d.0 = digits[min_index];
        }
        if let Some((max_index, _)) = positions.iter().enumerate().max_by_key(|&(_, pos)| pos)
        {
            d.1 = digits[max_index];
        }
        result.push(10*d.0 + d.1);
    }
    result.iter().sum()
}

fn main()
{
    println!("{}", day1_1());
    println!("{}", day1_2());
}
