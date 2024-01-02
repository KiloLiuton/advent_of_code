use common::read_lines;

fn day2_1() -> u32
{
    let total = Sample { red: 12, green: 13, blue: 14 };
    let mut game_ids = Vec::new();
    'game_loop: for line in read_lines("inputs/in2.txt")
    {
        let gid = get_game_id(&line);
        let samples = get_game_samples(&line);
        // print!("{}: ", gid);
        for sample in samples
        {
            // print!(" (r:{:<2} g:{:<2} b:{:<2})", sample.red, sample.green, sample.blue);
            if (sample.red > total.red) | (sample.green > total.green) | (sample.blue > total.blue)
            {
                // println!("<-IMPOSSIBRU ");
                continue 'game_loop
            }
        }
        // println!("");
        game_ids.push(gid);
    }
    game_ids.iter().sum()
}

fn day2_2() -> u32
{
    let mut total_power: u32 = 0;
    for line in &read_lines("inputs/in2.txt")
    {
        let mut min_sample = Sample { red: 0, green: 0, blue: 0 };
        let samples = get_game_samples(&line);
        for sample in samples
        {
            min_sample.red = std::cmp::max(min_sample.red, sample.red);
            min_sample.green = std::cmp::max(min_sample.green, sample.green);
            min_sample.blue = std::cmp::max(min_sample.blue, sample.blue);
        }
        total_power += min_sample.power();
    }
    total_power
}

fn get_game_id(line: &str) -> u32
{
    if let Some(prefix) = line.split(':').next()
    {
        if let Some(gid) = prefix.split(' ').last()
        {
            return gid.parse::<u32>().unwrap()
        }
    }
    1000000000
}

struct Sample
{
    red: u32,
    green: u32,
    blue: u32,
}

impl Sample
{
    fn power(&self) -> u32
    {
        self.red * self.green * self.blue
    }
}

fn get_game_samples(line: &str) -> Vec<Sample>
{
    let rhs = line.split(':').last().unwrap();
    let games = rhs.split(';');
    let mut results = Vec::new();

    for game in games
    {
        let samples = game.trim().split(',');
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for sample in samples
        {
            let num = sample.trim().split(' ').next().unwrap().parse::<u32>().unwrap();
            let color = sample.trim().split(' ').last().unwrap();
            match color
            {
                "red" => red = num,
                "green" => green = num,
                "blue" => blue = num,
                _ => println!("This shouldn't have happened!")
            }
        }
        results.push(Sample { red, green, blue });
    }
    results
}

fn main()
{
    println!("Answer part 1: {}", day2_1());
    println!("Answer part 2: {}", day2_2());
}
