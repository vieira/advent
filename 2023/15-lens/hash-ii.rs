use std::io;

fn hash(step: &str) -> usize {
    let mut code = 0;
    for c in step.chars() {
        code += c as usize;
        code *= 17;
        code %= 256;
    }

    code
}

type Lens = (String, u32);

struct HashMap {
    boxes: Vec<Vec<Lens>>,
}

impl HashMap {
    fn new() -> Self {
        HashMap { boxes: (0..256).map(|_| vec![]).collect() }
    }

    fn insert(&mut self, label: String, flen: u32) {
        let b = &mut self.boxes[hash(&label)];
        if let Some(idx) = b.iter().position(|(l, _)| label == *l) {
            b[idx] = (label, flen);
            return;
        }
        b.push((label, flen));
    }

    fn remove(&mut self, label: &str) {
        let b = &mut self.boxes[hash(label)];
        if let Some(idx) = b.iter().position(|(l, _)| label == *l) {
            b.remove(idx);
        }
    }

    fn focusing_power(&self) -> usize {
        self.boxes
            .iter()
            .enumerate()
            .map(|(i, b)| {
                b.iter()
                    .enumerate()
                    .fold(0, |acc, (j, (_, flen))| {
                        acc + (i + 1) * (j + 1) * (*flen as usize)
                    })
            })
            .sum()
    }
}

fn proccess_lens(lens: &str, lenses: &mut HashMap) {
    match lens {
        _ if lens.ends_with('-') => {
            let label = &lens[..lens.len() - 1];
            lenses.remove(label);
        },
        _ => {
            let (label, flen) = lens.split_once('=').unwrap();
            lenses.insert(label.to_string(), flen.parse().unwrap());
        }
    }
}

fn main() {
    let line = io::stdin().lines().next().unwrap().unwrap();
    let mut lenses = HashMap::new();

    for lens in line.split(',') {
        proccess_lens(lens, &mut lenses);
    }

    let power = lenses.focusing_power();
    println!("{power}");
}
