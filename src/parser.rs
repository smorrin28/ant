use crate::simulation::Command;

pub fn get_commands(content: &String) -> Vec<Command> {
    let mut current_index = 0;
    let mut commands: Vec<Command> = vec![];
    for line in content.split("\n") {
        let splitted: Vec<&str> = line.split(":").collect();
        let index: i32 = splitted[0].parse().expect("Invalid index!");
        let command_content: String = splitted[1].to_string();
        if index != current_index + 1 {
            panic!("Invalid index!");
        }
        current_index += 1;
        let command = get_command_enum(command_content);
        commands.push(command);
    }
    commands
}

fn get_command_enum(command: String) -> Command {
    let splitted: Vec<&str> = command.split_whitespace().collect();
    let cmd = splitted[0];
    let arg = splitted.get(1);
    if cmd != "END" && arg.is_none() {
        panic!("Invalid argumtent!");
    } else if cmd == "END" && arg.is_none() {
        return Command::END;
    }
    let n = arg.unwrap().parse::<usize>().expect("Argument must be a number!");

    match cmd {
        "DLOAD" => Command::DLOAD(n),
        "LOAD" => Command::LOAD(n),
        "STORE" => Command::STORE(n),
        "ADD" => Command::ADD(n),
        "SUB" => Command::SUB(n),
        "MULT" => Command::MULT(n),
        "DIV" => Command::DIV(n),
        "JUMP" => Command::JUMP(n),
        "JGT" => Command::JGT(n),
        "JGE" => Command::JGE(n),
        "JEQ" => Command::JEQ(n),
        "JLE" => Command::JLE(n),
        "JLT" => Command::JLT(n),
        _ => panic!("Invalid command!"),
    }
}
