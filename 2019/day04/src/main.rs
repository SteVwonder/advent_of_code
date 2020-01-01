fn get_digits(candidate: u32) -> Vec<u32> {
    fn inner(n: u32, digits: &mut Vec<u32>) {
        if n >= 10 {
            inner(n / 10, digits);
        }
        digits.push(n % 10);
    }
    let mut digits = Vec::new();
    inner(candidate, &mut digits);
    digits
}

fn two_adjacent(digits: &Vec<u32>) -> bool {
    digits.iter().zip(digits[1..].iter()).any(|(a,b)| {
        a == b
    })
}

fn non_decreasing(digits: &Vec<u32>) -> bool {
    digits.iter().zip(digits[1..].iter()).all(|(a,b)| {
        a <= b
    })
}

fn only_two(digits: &Vec<u32>) -> bool {
    let mut curr_digit = digits[0];
    let mut count = 1;
    for digit in digits[1..].iter() {
        if *digit == curr_digit {
            count += 1;
        } else if count == 2 {
            return true;
        } else {
            count = 1;
            curr_digit = *digit;
        }
    }
    return count == 2;
}

fn solve_part1(candidates: &Vec<u32>) -> u32{
    candidates.iter().filter(|candidate: u32| {
        let digits = get_digits(candidate);
        two_adjacent(&digits) && non_decreasing(&digits)
    }).count() as u32
}

fn solve_part2(candidates: &Vec<u32>) -> u32{
    let valid_numbers: Vec<&u32> = candidates.iter().filter(|candidate| {
        let digits = get_digits(**candidate);
        only_two(&digits) && non_decreasing(&digits)
    }).collect();
    valid_numbers.len() as u32
}

fn main() {
    let candidates: Vec<u32> = (134564..585160).collect();
    println!("Part 1: {}", solve_part1(&candidates));
    println!("Part 2: {}", solve_part2(&candidates));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_adjacent() {
        assert_eq!(two_adjacent(&vec![0, 1, 1, 2, 3, 4]), true);
        assert_eq!(two_adjacent(&vec![0, 1, 2, 3, 4, 4]), true);
        assert_eq!(two_adjacent(&vec![0, 1, 2, 3, 4, 5]), false);
    }

    #[test]
    fn test_groups_of_two() {
        assert_eq!(only_two(&vec![0, 1, 1, 2, 3, 4]), true);
        assert_eq!(only_two(&vec![0, 1, 2, 3, 4, 4]), true);
        assert_eq!(only_two(&vec![0, 1, 2, 4, 4, 4]), false);
        assert_eq!(only_two(&vec![0, 1, 4, 4, 4, 4]), false);
        assert_eq!(only_two(&vec![0, 1, 2, 3, 4, 5]), false);
        assert_eq!(only_two(&vec![4, 4, 4, 4, 4, 4]), false);
    }

    #[test]
    fn test_non_decreasing() {
        assert_eq!(non_decreasing(&vec![0, 1, 1, 2, 3, 4]), true);
        assert_eq!(non_decreasing(&vec![0, 1, 2, 3, 4, 4]), true);
        assert_eq!(non_decreasing(&vec![0, 1, 2, 3, 4, 5]), true);
        assert_eq!(non_decreasing(&vec![6, 6, 6, 6, 6, 6]), true);
        assert_eq!(non_decreasing(&vec![6, 6, 6, 6, 6, 0]), false);
        assert_eq!(non_decreasing(&vec![6, 0, 6, 6, 6, 6]), false);
    }
}
