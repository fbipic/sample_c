use std::env; //Include to use arguments passed via command line
use std::fs; //Include to use files

pub fn run(name: &str){
    println!("==>Welcome to demo_io {}!!!", String::from(name));

    cmd_line_args_example();
    read_file_example();
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

fn read_file_example() {
    let contents = fs::read_to_string("planets.txt").unwrap();
    println!("contents=<{}>", contents);

    for (index, line) in contents.lines().enumerate(){
        println!("line {} = {}", index, line);
    }

    let contents = fs::read("planets.txt").unwrap();
    println!("contents=<{:?}>", contents);   
}