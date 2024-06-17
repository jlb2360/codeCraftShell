use std::env;

struct Command {
    name: &'static str,
    function: fn(Vec<&str>),
}

pub struct Commands {
    built_in: Vec<Command>,
    path: Vec<String>,
}

impl Commands {
    pub fn new() -> Commands {
        let path = env::var("PATH").unwrap();
        Commands {
            built_in: Vec::new(),
            path: path.split(":").map(|s| s.to_string()).collect(),
        }
    }


    

    pub fn add_built_in_commands(&mut self) {
        self.built_in.push(Command {
            name: "exit",
            function: exit_command,
        });

        self.built_in.push(Command {
            name: "echo",
            function: echo_command,
        });
    }

    pub fn execute_command(&self, name: &str, params: Vec<&str>) {
        for command in &self.built_in {
            if command.name == name {
                (command.function)(params);
                return;
            }
        }

        if name == "type" {
            self.type_command(params);
            return;
        }

        println!("{}: command not found", name);
    }

    fn type_command(&self, params: Vec<&str>) {
        let command = params[0];
        for c in &self.built_in {
            if c.name == command {
                println!("{} is a shell builtin", command);
                return;
            }
        }
        if command == "type" {
            println!("type is a shell builtin");
            return;
        }

        for path in &self.path {
            let full_path = format!("{}/{}", path, command);
            if std::path::Path::new(&full_path).exists() {
                println!("{} is {}", command, full_path);
                break;
            }
        }


        println!("{}: not found", command);
    }
    
}

fn exit_command(params: Vec<&str>) {
    if params.len() > 2 {
        println!("exit: too many arguments");
        return;
    }

    if params.len() == 2 {
        let code = params[1].parse::<i32>();
        match code {
            Ok(_) => std::process::exit(code.unwrap()),
            Err(_) => println!("exit: {}: numeric argument required", params[1]),
        }
    } else {
        std::process::exit(0);
    }
}

fn echo_command(params: Vec<&str>) {
    println!("{}", params.join(" "));
}