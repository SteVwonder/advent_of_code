use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use intcode::{Intcode,parse_line};

fn main() {
    let file = File::open(Path::new("./input")).unwrap();
    let reader = BufReader::new(file);
    let line = reader.lines().next().unwrap().unwrap();
    let program = parse_line(&line);
    let mut state_v1 = Intcode::new(program.clone(), vec![1]);
    state_v1.execute(false);
    println!("Part 1: {}", state_v1.output_iter().last().unwrap());
    let mut state_v2 = Intcode::new(program.clone(), vec![5]);
    state_v2.execute(false);
    println!("Part 2: {}", state_v2.output_iter().next().unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn answers() {
        let file = File::open(Path::new("./input")).unwrap();
        let reader = BufReader::new(file);
        let line = reader.lines().next().unwrap().unwrap();
        let program = parse_line(&line);

        let mut state_v1 = Intcode::new(program.clone(), vec![1]);
        state_v1.execute(false);
        let mut output = state_v1.output_iter().rev();
        assert!(*(output.next().unwrap()) == 16209841);
        for &output in output {
            assert!(output == 0);
        }

        let mut state_v2 = Intcode::new(program.clone(), vec![5]);
        state_v2.execute(false);
        assert!(*(state_v2.output_iter().next().unwrap()) == 8834787);
    }
}
