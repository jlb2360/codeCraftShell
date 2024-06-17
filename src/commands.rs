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

        self.built_in.push(Command {
            name: "pwd",
            function: |_| {
                println!("{}", env::current_dir().unwrap().display());
            },
        });

        self.built_in.push(Command{
            name: "cd",
            function: cd_command,
        })
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

        for path in &self.path {
            let full_path = format!("{}/{}", path, name);
            if std::path::Path::new(&full_path).exists() {
                self.execute_external_command(&full_path, params);
                return;
            }
        }

        println!("{}: command not found", name);
    }

    fn execute_external_command(&self, full_path: &str, params: Vec<&str>) {
        let mut command = std::process::Command::new(full_path);
        command.args(params);
        let status = command.status().expect("failed to execute process");
        if !status.success() {
            println!("{}: command failed with status {}", full_path, status);
        }
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

        let mut found = false;
        for path in &self.path {
            let full_path = format!("{}/{}", path, command);
            if std::path::Path::new(&full_path).exists() {
                println!("{} is {}", command, full_path);
                found = true;
                break;
            }
        }

        if !found {
            println!("{}: not found", command);
        }

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

fn cd_command(params: Vec<&str>){    
    if params.len() > 2 {
        println!("cd: too many arguments");
        return;
    }

    if params.len() <= 0 {
        println!("cd: missing argument")
    } else {
        let path = params[0];
        if std::path::Path::new(path).exists() {
            env::set_current_dir(path).unwrap();
        } else {
            // check for ~
            if path.starts_with("~"){
                let home = match env::var("HOME"){
                    Ok(val) => val,
                    Err(_) => {
                        println!("cd: HOME not set");
                        return;
                    }
                };
                let new_path = home + &path[1..];
                if std::path::Path::new(&new_path).exists() {
                    env::set_current_dir(new_path).unwrap();
                } else {
                    println!("cd: {}: No such file or directory", path);
                }
                return;
            }

            // try relative path
            let current_dir = env::current_dir().unwrap();
            let new_path = current_dir.join(path);
            if std::path::Path::new(&new_path).exists() {
                env::set_current_dir(new_path).unwrap();
            } else {
                println!("cd: {}: No such file or directory", path);
            }
        }
    }
}