use std::process::exit;

#[derive(Debug, Copy, Clone)]
pub enum Command {
    DLOAD(usize),
    LOAD(usize),
    STORE(usize),
    ADD(usize),
    SUB(usize),
    MULT(usize),
    DIV(usize),
    JUMP(usize),
    JGT(usize),
    JGE(usize),
    JEQ(usize),
    JLE(usize),
    JLT(usize),
    END,
}

pub struct Simulation {
    pub commands: Vec<Command>,
    pub accumulator: usize,
    pub counter: usize,
    pub registers: [usize; 16],
}

impl Simulation {
    pub fn run(&mut self) {
        loop {
            let command = self.commands[self.counter - 1];
            match command {
                Command::DLOAD(n) => self.accumulator = n,
                Command::LOAD(n) => self.accumulator = self.registers[n - 1],
                Command::STORE(n) => self.registers[n - 1] = self.accumulator,
                Command::ADD(n) => self.accumulator += self.registers[n - 1],
                Command::SUB(n) => self.accumulator -= self.registers[n - 1],
                Command::MULT(n) => self.accumulator *= self.registers[n - 1],
                Command::DIV(n) => self.accumulator /= self.registers[n - 1],
                Command::JUMP(n) => { self.counter = n; continue },
                Command::JGT(n) =>  if self.accumulator > 0 { self.counter = n; continue },
                Command::JGE(n) => if self.accumulator >= 0 { self.counter = n; continue },
                Command::JEQ(n) => if self.accumulator == 0 { self.counter = n; continue },
                Command::JLE(n) => if self.accumulator <= 0 { self.counter = n; continue },
                Command::JLT(n) => if self.accumulator < 0 { self.counter = n; continue },
                Command::END => self.end(),
            }
            self.counter += 1;
        }
    }

    fn end(&self) {
        println!("=== Program ended ===");
        println!("Counter: {}", self.counter);
        println!("Accumulator: {}", self.accumulator);
        for i in 0..16 {
            print!("R{}: {} ", i + 1, self.registers[i]);
        }
        println!("");
        println!("======");
        exit(0);
    }
}
