use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn fuel_calc_v2(score: i32) -> i32 {
    let fuel_req = fuel_calc_v1(score);
    if fuel_req < 0 {
        return 0;
    }
    fuel_req + fuel_calc_v2(fuel_req)
}

fn fuel_calc_v1(score: i32) -> i32 {
    (score as f32 / 3.0).floor() as i32 - 2
}

fn solve(path: &Path, fuel_calc: fn(i32) -> i32, verbose: bool) -> i32 {
    let display = path.display();
    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);
    let scores = reader.lines().map(|line| {
        line.unwrap().parse::<i32>().unwrap()
    });
    let fuel_reqs = scores.map(|score| {
        let fuel_req = fuel_calc(score);
        if verbose {
            println!("{} requires {} fuel", score, fuel_req);
        }
        fuel_req
    });
    fuel_reqs.sum()
}

fn main() {
    for (name, path, verbose) in [
        ("Test", Path::new("./test"), true),
        ("Real", Path::new("./input"), false),
    ]
    .iter()
    {
        println!("======={} Case=======", name);
        for (idx, func) in [fuel_calc_v1, fuel_calc_v2].iter().enumerate() {
            println!("==Part {}==", idx + 1);
            println!("Total Fuel Requirements: {}", solve(path, *func, *verbose));
        }
    }
}
