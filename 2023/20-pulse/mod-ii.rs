use std::io;
use std::cmp;
use std::collections::HashMap;

fn gcd(a: usize, b: usize) -> usize {
    let mut num = cmp::max(a, b);
    let mut div = cmp::min(a, b);

    while div != 0 {
        let rem = num % div;
        num = div;
        div = rem;
    }
    num
}

fn lcm(nums: &[usize]) -> usize {
    nums.iter().copied().reduce(|acc, v| acc * v / gcd(acc, v)).unwrap()
}

#[derive(Debug, Clone)]
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

fn send_pulse(tgt: &str, mods: &mut HashMap<String, Module>) -> bool {
    let mut queue = vec![
        ("button".to_string(), "broadcaster".to_string(), false),
    ];

    while let Some((from, to, pulse)) = queue.pop() {
        if from == tgt && pulse { return false };

        let module = mods.get_mut(&to).unwrap();

        let output = module.receive(&from, pulse);

        if let Some(out_pulse) = output {
            for out_name in module.outputs.iter() {
                let to = to.to_owned();
                let out_name = out_name.to_owned();
                queue.insert(0, (to, out_name, out_pulse));
            }
        }
    }

    true
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

    let mut module = modules.get("rx").unwrap();
    while module.inputs.len() == 1 {
        let name = module.inputs.keys().next().unwrap();
        module = modules.get(&name[..]).unwrap();
    }

    let cycles: Vec<usize> = module.inputs.keys()
        .map(|name| {
            let mut mods = modules.clone();
            (1..).find(|_| !send_pulse(name, &mut mods)).unwrap()
        })
        .collect();


    let pushes = lcm(&cycles);
    println!("{pushes}");
}
