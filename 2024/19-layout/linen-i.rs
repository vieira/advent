use std::io;

fn can_arrange(pattern: &str, stripes: &[&str]) -> bool {
    if pattern.is_empty() {
        return true;
    }
    stripes.iter().any(|s| {
        pattern.starts_with(s) && can_arrange(&pattern[s.len()..], stripes)
    })
}

fn main() {
    let mut lines = io::stdin().lines();
    let line = lines.next().unwrap().unwrap();
    let stripes: Vec<_> = line.split(", ").collect();
    lines.next();

    let count = lines
        .map(|line| line.unwrap())
        .filter(|pattern| can_arrange(pattern, &stripes))
        .count();

    println!("{count}");
}
