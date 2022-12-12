use std::fs;

pub fn get_range(list: &str) -> (i32, i32) {
    let mut it = list.split("-");
    let first: i32 = it.next().unwrap().parse().unwrap();
    let last: i32 = it.next().unwrap().parse().unwrap();
    (first, last)
}

pub fn contains(a: (i32, i32), b: (i32, i32)) -> bool {
    if a.0 <= b.0 && a.1 >= b.1 {
        // a contains b
        return true;
    } else if b.0 <= a.0 && b.1 >= a.1 {
        // b contains a
        return true;
    }
    false
}

pub fn overlaps(a: (i32, i32), b: (i32, i32)) -> bool {
    if contains(a, b) {
        return true;
    } else if a.0 >= b.0 && a.0 <= b.1 {
        // left corner of a inside b
        return true;
    } else if a.1 >= b.0 && a.1 <= b.1 {
        // right corner of a inside b
        return true;
    }
    false
}

pub fn solution() -> Result<(), String> {
    let sections = &fs::read_to_string("inputs/day4").expect("Can't open file!");
    let sections: Vec<&str> = sections.split_terminator("\n").collect();

    let mut num_contains = 0;
    let mut num_overlaps = 0;
    for section in sections {
        let mut it = section.split(",");
        let sections_left = it.next().expect("Can't parse first elf!");
        let sections_right = it.next().expect("Can't parse second elf!");

        let range1 = get_range(sections_left);
        let range2 = get_range(sections_right);

        if contains(range1, range2) {
            num_contains += 1;
        }
        if overlaps(range1, range2) {
            num_overlaps += 1;
        }
    }
    println!("PART 1:\nnumber of full overlaps: {}", num_contains);
    println!("PART 2:\nnumber of overlaps: {}", num_overlaps);

    Ok(())
}
