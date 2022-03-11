mod simulation;
mod parser;

use std::fs::read_to_string;

fn read_file(path: &String) -> String {
    let content = read_to_string(path).expect("File not found.");
    content
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let my_file = read_file(&args[1]);
    let commands = parser::get_commands(&my_file);
    let mut s = simulation::Simulation {
        commands: commands,
        accumulator: 0,
        counter: 1,
        registers: [0; 16],
    };
    s.run();
}
