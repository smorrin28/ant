use std::process::exit;

#[derive(Debug, Copy, Clone)]
pub enum Command {
    DLOAD(i32),
    LOAD(i32),
    STORE(i32),
    ADD(i32),
    SUB(i32),
    MULT(i32),
    DIV(i32),
    JUMP(i32),
    JGT(i32),
    JGE(i32),
    JEQ(i32),
    JLE(i32),
    JLT(i32),
    END,
}

pub struct Simulation {
    pub commands: Vec<Command>,
    pub accumulator: i32,
    pub counter: i32,
    pub registers: [i32; 16],
}

impl Simulation {
    pub fn run(&mut self) {
        loop {
            let command = self.commands[(self.counter - 1) as usize];
            match command {
                Command::DLOAD(n) => self.accumulator = n,
                Command::LOAD(n) => self.accumulator = self.registers[(n - 1) as usize],
                Command::STORE(n) => self.registers[(n - 1) as usize] = self.accumulator,
                Command::ADD(n) => self.accumulator += self.registers[(n - 1) as usize],
                Command::SUB(n) => self.accumulator -= self.registers[(n - 1) as usize],
                Command::MULT(n) => self.accumulator *= self.registers[(n - 1) as usize],
                Command::DIV(n) => self.accumulator /= self.registers[(n - 1) as usize],
                Command::JUMP(n) => {
                    self.counter = n;
                    continue;
                }
                Command::JGT(n) => {
                    if self.accumulator > 0 {
                        self.counter = n;
                        continue;
                    }
                }
                Command::JGE(n) => {
                    if self.accumulator >= 0 {
                        self.counter = n;
                        continue;
                    }
                }
                Command::JEQ(n) => {
                    if self.accumulator == 0 {
                        self.counter = n;
                        continue;
                    }
                }
                Command::JLE(n) => {
                    if self.accumulator <= 0 {
                        self.counter = n;
                        continue;
                    }
                }
                Command::JLT(n) => {
                    if self.accumulator < 0 {
                        self.counter = n;
                        continue;
                    }
                }
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
