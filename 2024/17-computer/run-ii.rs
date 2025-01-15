use std::io;

#[derive(Debug)]
struct Device {
    registers: Vec<u64>,
    output: Vec<u64>,
    ip: usize,
}

impl Device {
    fn new(registers: Vec<u64>) -> Self {
        Self {
            registers,
            output: vec![],
            ip: 0,
        }
    }

    fn combo(&self, operand: u64) -> u64 {
        match operand {
            0..=3 => operand,
            4..=6 => self.registers[operand as usize - 4],
            _ => panic!("invalid operand {}", operand),
        }
    }

    fn div(&self, operand: u64) -> u64 {
        self.registers[0] / (1 << self.combo(operand))
    }

    fn adv(&mut self, operand: u64) {
        self.registers[0] = self.div(operand);
    }

    fn bdv(&mut self, operand: u64) {
        self.registers[1] = self.div(operand);
    }

    fn cdv(&mut self, operand: u64) {
        self.registers[2] = self.div(operand);
    }

    fn bxl(&mut self, operand: u64) {
        self.registers[1] ^= operand;
    }

    fn bxc(&mut self, _: u64) {
        self.registers[1] ^= self.registers[2];
    }

    fn bst(&mut self, operand: u64) {
        self.registers[1] = self.combo(operand) % 8;
    }

    fn jnz(&mut self, operand: u64) {
        match self.registers[0] {
            0 => self.ip += 2,
            _ => self.ip = operand as usize,
        }
    }

    fn out(&mut self, operand: u64) {
        self.output.push(self.combo(operand) % 8);
    }

    fn run(&mut self, program: &[u64]) -> &[u64] {
        if self.ip >= program.len() {
            return &self.output;
        }

        let [opcode, operand] = program[self.ip..self.ip + 2] else {
            unreachable!()
        };

        match opcode {
            0 => self.adv(operand),
            1 => self.bxl(operand),
            2 => self.bst(operand),
            3 => self.jnz(operand),
            4 => self.bxc(operand),
            5 => self.out(operand),
            6 => self.bdv(operand),
            7 => self.cdv(operand),
            _ => panic!("invalid opcode {}", opcode),
        }

        match opcode {
            3 => {}
            _ => self.ip += 2,
        }

        self.run(program)
    }
}

fn find_a(program: &[u64], registers: &[u64]) -> u64 {
    let mut i = 1;

    loop {
        let initial_registers = registers
            .iter()
            .enumerate()
            .map(|(id, v)| if id == 0 { i } else { *v })
            .collect();

        let mut device = Device::new(initial_registers);
        let output = device.run(program);

        if output == program {
            return i;
        }

        if output.iter().rev().zip(program.iter().rev()).all(|(a, b)| a == b) {
            i *= 8;
            continue;
        }

        i += 1;
    }
}

fn main() {
    let mut lines = io::stdin().lines();
    let registers: Vec<u64> = lines
        .by_ref()
        .map(|line| line.unwrap())
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let (_, value) = line.split_once(": ").unwrap();
            value.parse().unwrap()
        })
        .collect();

    let line = lines.next().unwrap().unwrap();
    let (_, program) = line.split_once(": ").unwrap();
    let program: Vec<u64> =
        program.split(',').map(|c| c.parse().unwrap()).collect();

    println!("{}", find_a(&program, &registers));
}
