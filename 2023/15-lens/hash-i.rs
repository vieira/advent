use std::io;

fn hash(step: &str) -> u32 {
    let mut code = 0;
    for c in step.chars() {
        code += c as u32;
        code *= 17;
        code %= 256;
    }

    code
}

fn main() {
    let line = io::stdin().lines().next().unwrap().unwrap();
    let sum: u32 = line.split(',').map(hash).sum();
    println!("{sum}");
}
