use std::env; //Include to use arguments passed via command line
use std::fs; //Include to use files
//use std::io::Write; //Include to allow to write to file
use std::io::prelude::*; //Include to allow to read, write to file

pub fn run(name: &str){
    println!("==>Welcome to demo_io {}!!!", String::from(name));

    cmd_line_args_example();
    read_file_example();
    write_file_example();

    roster_challenge();
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

    //Read file bytes
    let contents = fs::read("planets.txt").unwrap();
    println!("contents=<{:?}>", contents);   
}

fn write_file_example() {
    let mut speech = String::new();
    speech.push_str("We choose to go to the Moon in this decade\n");
    speech.push_str("and do the other things,\n");
    speech.push_str("not because they are easy,\n");
    speech.push_str("but because they are hard.");

    //This function writes all contents in one time and overwrite existing file
    fs::write("speech.txt", speech);

    let mut file = fs::OpenOptions::new().append(true).open("planets.txt").unwrap();
    file.write(b"\nPluto");
}

fn roster_challenge() {
    if env::args().len() != 2 {
        println!("Two arguments required!")
    }

    let file_path = env::args().nth(1).unwrap();
    println!("file={}", file_path);

    let astronaut = env::args().nth(2).unwrap();
    println!("Search astronauts={} in file={}", astronaut, file_path);

    let mut not_found= true;
    for (index, line) in fs::read_to_string(file_path).unwrap().lines().enumerate(){
        if line == astronaut { 
            println!("Find astronauts {} on line {}", astronaut, index);
            not_found = false;
            break; 
        }
    }
    if not_found { println!("{} did not walk on Moon...", astronaut); }

}