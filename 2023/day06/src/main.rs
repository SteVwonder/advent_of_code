use std::env;

fn num_ways_to_win(time: u32, dist: u32) -> u32 {
    (1..time).map(|t| t * (time - t))
        .skip_while(|d| { *d <= dist})
        .take_while(|d| { *d > dist}).count() as u32
}

fn part1(lines: Vec<String>) {
    let mut it = lines.iter();
    let mut times_split = it.next().unwrap().split(" ");
    assert!(times_split.next().unwrap() == "Time:");
    let times = times_split.filter_map(|x| { x.parse::<u32>().ok() });

    let mut dist_split = it.next().unwrap().split(" ");
    assert!(dist_split.next().unwrap() == "Distance:");
    let dists = dist_split.filter_map(|x| { x.parse::<u32>().ok() });

    let score: u32 = times.zip(dists)
        .map(|(x,y)| {num_ways_to_win(x,y)})
        .fold(1, |accum, x| { accum * x });
    println!("Part1: {}", score);
}

fn part2(lines: Vec<String>) {
}

fn read_file_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in std::fs::read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];

    let lines = read_file_lines(filename);
    part1(lines.clone());
    part2(lines.clone());
}
