use std::env;

fn num_ways_to_win(time: u64, dist: u64) -> u64 {
    (1..time).map(|t| t * (time - t))
        .skip_while(|d| { *d <= dist})
        .take_while(|d| { *d > dist}).count() as u64
}

fn part1(lines: Vec<String>) {
    let mut it = lines.iter();
    let mut times_split = it.next().unwrap().split(" ");
    assert!(times_split.next().unwrap() == "Time:");
    let times = times_split.filter_map(|x| { x.parse::<u64>().ok() });

    let mut dist_split = it.next().unwrap().split(" ");
    assert!(dist_split.next().unwrap() == "Distance:");
    let dists = dist_split.filter_map(|x| { x.parse::<u64>().ok() });

    let score: u64 = times.zip(dists)
        .map(|(x,y)| {num_ways_to_win(x,y)})
        .fold(1, |accum, x| { accum * x });
    println!("Part1: {}", score);
}

fn strip_prefix_and_merge(line: &String) -> u64 {
   let (_, suffix) = line.split_once(" ").unwrap();
    suffix.replace(" ", "").parse::<u64>().unwrap()
}

fn part2(lines: Vec<String>) {
    let mut it = lines.iter();
    let time = strip_prefix_and_merge(it.next().unwrap());
    let dist = strip_prefix_and_merge(it.next().unwrap());

    let score: u64 = num_ways_to_win(time, dist);
    println!("Part2: {}", score);
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
