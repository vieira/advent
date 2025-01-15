use std::io;

#[derive(Debug)]
struct Device {
    registers: Vec<u32>,
    output: Vec<u32>,
    ip: usize,
}

impl Device {
    fn new(registers: Vec<u32>) -> Self {
        Self {
            registers,
            output: vec![],
            ip: 0,
        }
    }

    fn combo(&self, operand: u32) -> u32 {
        match operand {
            0..=3 => operand,
            4..=6 => self.registers[operand as usize - 4],
            _ => panic!("invalid operand {}", operand),
        }
    }

    fn div(&self, operand: u32) -> u32 {
        self.registers[0] / (1 << self.combo(operand))
    }

    fn adv(&mut self, operand: u32) {
        self.registers[0] = self.div(operand);
    }

    fn bdv(&mut self, operand: u32) {
        self.registers[1] = self.div(operand);
    }

    fn cdv(&mut self, operand: u32) {
        self.registers[2] = self.div(operand);
    }

    fn bxl(&mut self, operand: u32) {
        self.registers[1] ^= operand;
    }

    fn bxc(&mut self, _: u32) {
        self.registers[1] ^= self.registers[2];
    }

    fn bst(&mut self, operand: u32) {
        self.registers[1] = self.combo(operand) % 8;
    }

    fn jnz(&mut self, operand: u32) {
        match self.registers[0] {
            0 => self.ip += 2,
            _ => self.ip = operand as usize,
        }
    }

    fn out(&mut self, operand: u32) {
        self.output.push(self.combo(operand) % 8);
    }

    fn run(&mut self, program: &[u32]) -> String {
        if self.ip >= program.len() {
            return self
                .output
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(",");
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

fn main() {
    let mut lines = io::stdin().lines();
    let registers: Vec<u32> = lines
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
    let program: Vec<u32> =
        program.split(',').map(|c| c.parse().unwrap()).collect();

    let mut device = Device::new(registers);
    println!("{}", device.run(&program));
}
