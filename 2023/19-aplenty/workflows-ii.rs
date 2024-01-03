use std::io;
use std::ops::Range;
use std::collections::HashMap;

type Segment = Range<u32>;
type Part = HashMap<char, Segment>;
type Workflows = HashMap<String, Vec<Rule>>;

struct Condition {
    variable: char,
    operator: char,
    value: u32,
}

struct Rule {
    condition: Option<Condition>,
    target: String,
}

impl Rule {
    fn eval(&self, part: &Part) -> Vec<(String, Part)> {
        let target = self.target.to_owned();

        if let Some(condition) = &self.condition {
            let range = part.get(&condition.variable).unwrap();
            let mut p_tgt = part.clone();
            let mut p_next = part.clone();

            return match condition.operator {
                _ if range.contains(&condition.value) => {
                    let (r_tgt, r_next) = if condition.operator == '<' {
                        let first = range.start..condition.value;
                        let last = condition.value..range.end;

                        (first, last)
                    } else {
                        let first = condition.value + 1..range.end;
                        let last = range.start..condition.value + 1;

                        (first, last)
                    };

                    p_tgt.insert(condition.variable.to_owned(), r_tgt);
                    p_next.insert(condition.variable.to_owned(), r_next);
                    vec![(target, p_tgt), ("".to_string(), p_next)]
                },
                '>' if range.start > condition.value => {
                    vec![(target, part.clone())]
                },
                '<' if range.end <= condition.value => {
                    vec![(target, part.clone())]
                },
                _ => {
                    vec![("".to_string(), part.clone())]
                },
            }
        }

        vec![(target, part.clone())]
    }
}

fn parse_workflow(line: &str) -> (String, Vec<Rule>) {
    let (name, rest) = line.split_once('{').unwrap();
    let workflow = &rest[..rest.len() - 1];

    let rules = workflow.split(',')
        .map(|r| {
            if !r.contains(':') {
                return Rule { condition: None, target: r.to_owned() };
            }

            let (cond, target) = r.split_once(':').unwrap();
            let mut cond_chars = cond.chars();
            let variable = cond_chars.next().unwrap();
            let operator = cond_chars.next().unwrap();
            let value = cond[2..].parse().unwrap();

            Rule {
                condition: Some(Condition { variable, operator, value }),
                target: target.to_owned(),
            }
        })
        .collect();

        (name.to_owned(), rules)
}

fn eval_rules(part: &Part, rules: &[Rule]) -> Vec<(String, Part)> {
    let mut rest_part = part.to_owned();
    let mut parts = vec![];

    for rule in rules {
        for (tgt, next_part) in rule.eval(&rest_part) {
            if tgt.is_empty() {
                rest_part = next_part;
                continue;
            }
            parts.push((tgt, next_part));
        }
    }

    parts
}

fn combinations(part: &Part) -> u64 {
    part.values().map(|r| r.end as u64 - r.start as u64).product()
}

fn run_workflows(part: &Part, start: &str, workflows: &Workflows) -> u64 {
    let rules = workflows.get(start).unwrap();
    let mut sum = 0;

    for (tgt, p) in eval_rules(part, rules) {
        sum += match &tgt[..] {
            "A" => combinations(&p),
            "R" => 0,
            _ => run_workflows(&p, &tgt, workflows),
        }
    }

    sum
}

fn main() {
    let mut workflows = HashMap::new();

    for line in io::stdin().lines() {
        let line = line.unwrap();
        if line.is_empty() { break };
        let (name, rules) = parse_workflow(&line);
        workflows.insert(name, rules);
    }

    let part = HashMap::from([
        ('x', 1..4001),
        ('m', 1..4001),
        ('a', 1..4001),
        ('s', 1..4001),
    ]);

    let combinations = run_workflows(&part, "in", &workflows);
    println!("{combinations}");
}
