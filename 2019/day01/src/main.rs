use std::path::Path;
use std::io::{BufReader,BufRead};
use std::fs::File;

fn fuel_calc_v2(score: f32, debug: bool) -> f32 {
    let mut fuel_req = (score / 3.0).floor() - 2.0;
    if fuel_req < 0.0 {
        return 0.0;
    }
    fuel_req += fuel_calc_v2(fuel_req, false);
    if debug {
        println!("{} requires {} fuel", score, fuel_req);
    }
    fuel_req
}

fn fuel_calc_v1(score: f32, debug: bool) -> f32 {
    let fuel_req = (score / 3.0).floor() - 2.0;
    if debug {
        println!("{} requires {} fuel", score, fuel_req);
    }
    fuel_req as f32
}

fn solve(path: &Path, fuel_calc: fn(f32, bool) -> f32, debug: bool) -> u32{
    let display = path.display();
    let file = match File::open(&path) {
        Err(why) => panic!(
            "couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);
    let scores = reader.lines().map(|line| {
        let score: f32 = line.unwrap().parse().unwrap();
        score
    });
    let fuel_reqs = scores.map(|score| {
        fuel_calc(score, debug) as u32
    });
    let total : u32 = fuel_reqs.sum();
    return total;
}

fn main() {
    for (idx, func) in [fuel_calc_v1, fuel_calc_v2].iter().enumerate() {
        println!("======Part {}=====", idx+1);
        println!("==Test Case==");
        let mut answer : u32 = solve(Path::new("./test"), *func, true);
        println!("Total Fuel Requirements: {}", answer);
        println!("==Real Thing==");
        answer = solve(Path::new("./input"), *func, false);
        println!("Total Fuel Requirements: {}", answer);
    }
}
