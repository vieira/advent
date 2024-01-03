use std::io;
use std::collections::HashMap;

fn calc_calibration(line: &str, digits: &HashMap<&str, u8>) -> u32 {
    let mut first_digit = 0;
    let mut last_digit = 0;

    for (i, _) in line.chars().enumerate() {
        let fragment = &line[i..];
        let Some(digit) = digits.keys().find(|&d| fragment.starts_with(d))
        else { continue };

        let num = *digits.get(digit).unwrap();

        if first_digit == 0 {
            first_digit = num;
        }

        last_digit = num;
    }

    (first_digit * 10 + last_digit).into()
}

fn main() {
    let digits: HashMap<&str, u8> = HashMap::from([
       ("one", 1),
       ("two", 2),
       ("three", 3),
       ("four", 4),
       ("five", 5),
       ("six", 6),
       ("seven", 7),
       ("eight", 8),
       ("nine", 9),
       ("1", 1),
       ("2", 2),
       ("3", 3),
       ("4", 4),
       ("5", 5),
       ("6", 6),
       ("7", 7),
       ("8", 8),
       ("9", 9),
    ]);

    let result: u32 = io::stdin().lines()
        .map(|line| calc_calibration(&line.unwrap(), &digits))
        .sum();
    println!("{result}");
}
