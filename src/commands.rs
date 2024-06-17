

pub fn exit_command(params: Vec<&str>) {
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