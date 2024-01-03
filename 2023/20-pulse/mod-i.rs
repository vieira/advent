use std::io;
use std::collections::HashMap;

#[derive(Debug)]
struct Module {
    kind: char,
    state: bool,
    inputs: HashMap<String, bool>,
    outputs: Vec<String>,
}

impl Module {
    fn new(kind: char, outputs: Vec<String>) -> Self {
        Module {
            kind,
            state: false,
            inputs: HashMap::new(),
            outputs,
        }
    }

    fn set_kind(&mut self, kind: char) {
        self.kind = kind;
    }

    fn add_input(&mut self, name: &str) {
        self.inputs.insert(name.to_owned(), false);
    }

    fn receive(&mut self, name: &str, pulse: bool) -> Option<bool> {
        match self.kind {
            '%' => {
                if pulse { return None };
                self.state = !self.state;
                Some(self.state)
            },
            '&' => {
                self.inputs.insert(name.to_owned(), pulse);
                Some(self.inputs.values().any(|&x| !x))
            },
            _ => Some(pulse),
        }
    }
}

fn parse_line(line: &str) -> ((String, char), Vec<String>) {
    let (mut source, destination) = line.split_once(" -> ").unwrap();
    let outputs = destination.split(", ").map(|x| x.to_owned()).collect();
    let mut kind = source.chars().next().unwrap();

    if source == "broadcaster" {
        kind = '*';
    } else {
        source = &source[1..];
    }

    ((source.to_owned(), kind), outputs)
}

fn send_pulse(pulse: bool, mods: &mut HashMap<String, Module>) -> (u32, u32) {
    let mut queue = vec![
        ("button".to_string(), "broadcaster".to_string(), pulse),
    ];
    let mut sum_high = 0;
    let mut sum_low = 0;

    while let Some((from, to, pulse)) = queue.pop() {
        let module = mods.get_mut(&to).unwrap();

        if pulse { sum_high += 1 } else { sum_low += 1 };
        let output = module.receive(&from, pulse);

        if let Some(out_pulse) = output {
            for out_name in module.outputs.iter() {
                let to = to.to_owned();
                let out_name = out_name.to_owned();
                queue.insert(0, (to, out_name, out_pulse));
            }
        }
    }

    (sum_high, sum_low)
}

fn main() {
    let mut modules: HashMap<String, Module> = HashMap::new();

    for line in io::stdin().lines() {
        let ((name, kind), outputs) = parse_line(&line.unwrap());

        if let Some(module) = modules.get_mut(&name) {
            module.set_kind(kind);
            module.outputs = outputs.clone();
        } else {
            modules.insert(name.clone(), Module::new(kind, outputs.clone()));
        }

        for out_name in outputs {
            if let Some(module) = modules.get_mut(&out_name) {
                module.inputs.insert(name.clone(), false);
            } else {
                let mut module = Module::new('?', vec![]);
                module.add_input(&name);
                modules.insert(out_name, module);
            }
        }
    }

    let (sum_high, sum_low) = (0..1000)
        .map(|_| send_pulse(false, &mut modules))
        .fold((0, 0), |(acc_h, acc_l), (h, l)| (acc_h + h, acc_l + l));

    println!("{}", sum_high * sum_low);
}
