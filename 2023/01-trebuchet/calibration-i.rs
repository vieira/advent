use std::io;

fn calc_calibration(line: &str) -> u32 {
    let mut first_digit = 0;
    let mut last_digit = 0;

    for ch in line.chars() {
        if !ch.is_numeric() { continue }
        let num = ch.to_digit(10).unwrap();

        if first_digit == 0 {
            first_digit = num;
        }

        last_digit = num;
    }

    first_digit * 10 + last_digit
}

fn main() {
    let result: u32 = io::stdin().lines()
        .map(|line| calc_calibration(&line.unwrap()))
        .sum();
    println!("{result}");
}
