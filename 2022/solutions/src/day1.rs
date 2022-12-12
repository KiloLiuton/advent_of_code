use std::fs;

pub fn solution() -> std::io::Result<()> {
    let contents = fs::read_to_string("inputs/day1")?;
    let contents = contents.split("\n\n").collect::<Vec<&str>>();

    let mut bags: Vec<i32> = Vec::new();

    let mut max_cal = 0;
    let mut elf = 0;

    for (i, bag) in contents.iter().enumerate() {
        let calories: i32 = bag
            .split_terminator("\n")
            .map(|item| item.parse::<i32>().expect("Can't parse line!"))
            .fold(0, |acc, x| acc + x);
        if calories > max_cal {
            max_cal = calories;
            elf = i;
        }
        bags.push(calories);
    }

    println!("Part 1: elf {}, carrying {} calories", elf+1, max_cal);

    bags[elf] = 0;
    let mut top_three = max_cal;
    for _ in 0..2 {
        let mut max = 0;
        let mut elf = 0;
        for (i, &bag) in bags.iter().enumerate() {
            if bag > max {
                max = bag;
                elf = i;
            }
        }
        bags[elf] = 0;
        top_three += max;
    }
    println!("Part 2: three most caloric bags: {}", top_three);

    Ok(())
}
