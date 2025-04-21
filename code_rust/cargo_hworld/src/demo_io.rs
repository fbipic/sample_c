use std::env;

pub fn run(name: &str){
    println!("==>Welcome to demo_io {}!!!", String::from(name));

    cmd_line_args_example();
}

fn cmd_line_args_example() {
    if env::args().len() < 2 {
        println!("cmd_line_args_example requires at least 2 arguments!");
        return;
    }

    for (index, arg) in env::args().enumerate() {
        println!("arg[{}] = {}", index, arg);
    }

    let arg2 = env::args().nth(2).unwrap();
    println!("arg2={}", arg2);
}