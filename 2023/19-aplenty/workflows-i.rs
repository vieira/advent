use std::io;
use std::collections::HashMap;

type Part = HashMap<char, u32>;
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
    fn eval(&self, part: &Part) -> Option<String> {
        let target = self.target.to_owned();

        if let Some(condition) = &self.condition {
            let attribute = *part.get(&condition.variable).unwrap();

            return match condition.operator {
                '>' if attribute > condition.value => Some(target),
                '<' if attribute < condition.value => Some(target),
                _ => None
            }
        }

        Some(target)
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

fn parse_part(line: &str) -> Part {
    let part_line = &line[1..line.len() - 1];
    let mut part = HashMap::new();

    for attr in part_line.split(',') {
        let (name, value) = attr.split_once('=').unwrap();
        part.insert(name.chars().next().unwrap(), value.parse().unwrap());
    }

    part
}

fn run_workflows(part: &Part, start: &str, workflows: &Workflows) -> bool {
    let rules = workflows.get(start).unwrap();

    for rule in rules {
        if let Some(target) = rule.eval(part) {
            return match &target[..] {
                "A" => true,
                "R" => false,
                _ => run_workflows(part, &target, workflows),
            }
        }
    }

    false
}

fn main() {
    let mut workflows = HashMap::new();
    let mut lines = io::stdin().lines();

    for line in lines.by_ref() {
        let line = line.unwrap();
        if line.is_empty() { break };
        let (name, rules) = parse_workflow(&line);
        workflows.insert(name, rules);
    }

    let sum: u32 = lines.by_ref()
        .map(|line| parse_part(&line.unwrap()))
        .filter(|part| run_workflows(part, "in", &workflows))
        .map(|part| part.values().sum::<u32>())
        .sum();

    println!("{sum}");
}
