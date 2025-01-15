use std::collections::HashMap;
use std::io;

type Expr = (String, String, String);
type Exprs = HashMap<String, Expr>;

fn is_xy(lhs: &str, rhs: &str) -> bool {
    let lhs = lhs.chars().next().unwrap();
    let rhs = rhs.chars().next().unwrap();

    lhs == 'x' && rhs == 'y' || lhs == 'y' && rhs == 'x'
}

fn is_xor_with_input(expr: &Expr, input: &str) -> bool {
    let (lhs, op, rhs) = expr;
    op == "XOR" && (lhs == input || rhs == input)
}

fn is_or_with_input(expr: &Expr, input: &str) -> bool {
    let (lhs, op, rhs) = expr;
    op == "OR" && (lhs == input || rhs == input)
}

fn is_fault(output: &str, expr: &Expr, exprs: &Exprs) -> bool {
    let (lhs, op, rhs) = expr;

    if op != "XOR" && output.starts_with("z") {
        return true;
    }

    if op == "XOR" && !output.starts_with("z") && !is_xy(lhs, rhs) {
        return true;
    }

    if op == "XOR" && is_xy(lhs, rhs) {
        return !exprs.values().any(|expr| is_xor_with_input(expr, output));
    }

    if op == "AND" && ![&lhs[1..], &rhs[1..]].contains(&"00") {
        return !exprs.values().any(|expr| is_or_with_input(expr, output));
    }

    false
}

fn find_faults(exprs: &Exprs) -> Vec<String> {
    let min = exprs.keys().filter(|k| k.starts_with("z")).min();
    let max = exprs.keys().max();

    exprs
        .iter()
        .filter(|(output, _)| !min.iter().chain(&max).any(|b| b == output))
        .filter(|(output, expr)| is_fault(output, expr, exprs))
        .map(|(output, _)| output.to_owned())
        .collect()
}

fn main() {
    let mut lines = io::stdin().lines();

    let _ = lines
        .by_ref()
        .map(|line| line.unwrap())
        .take_while(|line| !line.is_empty())
        .count();

    let exprs: HashMap<String, Expr> = lines
        .map(|line| line.unwrap())
        .map(|line| {
            let (ex, out) = line.split_once(" -> ").unwrap();
            let [lhs, op, rhs] = &ex.split_whitespace().collect::<Vec<_>>()[..]
            else { panic!("bad expression") };

            let expr = (lhs.to_string(), op.to_string(), rhs.to_string());
            (out.to_owned(), expr)
        })
        .collect();

    let mut faults = find_faults(&exprs);
    faults.sort();
    println!("{}", faults.join(","));
}
