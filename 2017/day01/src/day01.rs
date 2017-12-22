#[macro_use]
extern crate log;
extern crate env_logger;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;


fn compare_and_cast (primary: char, secondary: char) -> u32 {
    let mut val: u32 = 0;

    debug!("{} -> {}", primary, secondary);
    if primary == secondary {
        debug!(" are equal");
        val = primary.to_string().parse::<u32>().unwrap();
    } else {
        debug!(" are not equal");
    }

    return val;
}

fn main() {
    let _logger = env_logger::init();

    let path = Path::new("data/input");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   why.description()),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut captcha = String::new();
    match file.read_to_string(&mut captcha) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   why.description()),
        Ok(_) => debug!("{}", captcha),
    }
    let captcha = captcha.trim();

    let mut sum: u32 = 0;
    for (primary_char, secondary_char) in captcha.chars().zip(captcha.chars().cycle().skip(1)) {
        sum = sum + compare_and_cast (primary_char, secondary_char);
        debug!("sum now == {}", sum);
    }

    print!("The answer to part1 is {}\n", sum);

    // Problem states that our captcha should be of even length, double check
    assert_eq! (captcha.len() % 2, 0);

    // Figure out the starting position of the 2nd iterator
    let half_captcha_len = captcha.len() / 2;
    let half_way_iter = captcha.chars().cycle().skip(half_captcha_len);

    let mut sum: u32 = 0;
    for (primary_char, secondary_char) in captcha.chars().take(half_captcha_len).zip(half_way_iter) {
        // Since the comparison is symmetric (we will compare index 0
        // with index n/2 and then later on index n/2 with index 0),
        // we can perform half of the comparisons and just double any
        // valid summations
        sum = sum + 2 * compare_and_cast (primary_char, secondary_char);
        debug!("sum now == {}", sum);
    }

    print!("The answer to part2 is {}\n", sum);
}
