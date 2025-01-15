use std::collections::HashMap;
use std::io;

type Expr = (String, String, String);
type Exprs = HashMap<String, Expr>;
type Values = HashMap<String, bool>;

fn output(ins: &Values, exprs: &Exprs, out: &str) -> bool {
    if let Some(val) = ins.get(out) {
        return *val;
    }

    let (lhs, op, rhs) = exprs.get(out).unwrap();
    let lhs = output(ins, exprs, lhs);
    let rhs = output(ins, exprs, rhs);

    match &op[..] {
        "AND" => lhs & rhs,
        "OR" => lhs | rhs,
        "XOR" => lhs ^ rhs,
        _ => unreachable!(),
    }
}

fn outputs(ins: &Values, exprs: &Exprs) -> usize {
    (0..)
        .map(|id| format!("z{:02}", id))
        .take_while(|id| exprs.contains_key(id))
        .map(|out| output(ins, exprs, &out))
        .enumerate()
        .fold(0, |acc, (i, v)| match v {
            true => acc + 2_usize.pow(i as u32),
            _ => acc,
        })
}

fn main() {
    let mut lines = io::stdin().lines();

    let ins: HashMap<String, bool> = lines
        .by_ref()
        .map(|line| line.unwrap())
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let (name, val) = line.split_once(": ").unwrap();
            (name.to_owned(), val.parse::<u8>().is_ok_and(|x| x > 0))
        })
        .collect();

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

    println!("{}", outputs(&ins, &exprs));
}
