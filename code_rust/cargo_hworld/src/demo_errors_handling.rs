use std::fs;
use std::io;
use rand::prelude::*;

pub fn run(name: &str){
    println!("==>Welcome to demo_errors_handling {}!!!", String::from(name));

    unrecoverable_error_example();
    recoverable_error_example();
    match_result_example();
    propagate_error_example();

    error_challenge();
}

//Recoverable errors -> enum Result<T,E>
//Unrecoverable errors -> panic! macro
fn unrecoverable_error_example() {
    //panic!("Houston, we have a problem!"); //This cause program to exit with 101 error

    let countdown = [5,4,3,2,1,0];
    for count in countdown.iter() {
        println!("T-minus {}", count);
        if *count == 0 { break;} //Added to avoid error 
        let x = 1 / count; //This will fails => divide by zero
    }
    //Use RUST_BACKTRACE=1 to get more information about error.
}

fn recoverable_error_example(){
    let content = fs::read_to_string("the_ultimate_question_result.txt");
    println!("Content is {:?}", content); //output of enum Result is displayed

    //We cannot extract value from enum Result directly so we can use .unwrap(), but in case of failure it does not work
    //let content = fs::read_to_string("the_ultimate_question_result.txt").unwrap(;
    //.expect() can be used to trigger a specific message in exception
    //let content = fs::read_to_string("the_ultimate_question_result.txt").expect("Nobody knows it");
}

fn match_result_example() {
    let result = fs::read_to_string("the_ultimate_question_result.txt");

    let contents = match result {
        Ok(message) => message,
        //Err(error) => String::from("Nobody knows it!")
        Err(error) => match error.kind() {
            io::ErrorKind::NotFound => String::from("File not found!"),
            io::ErrorKind::PermissionDenied => String::from("Permission denied!"),
            _ => panic!("There is another type of error")
        }
    };

    println!("Contents is {:?}", contents); //output of enum Result is displayed
}

fn read_and_combine(f1: &str, f2: &str) -> Result<String, io::Error> {
    let mut s1 = match fs::read_to_string(f1) {
        Ok(s) => s,
        Err(e) => return Err(e)
    };
/* 
    let s2 = match fs::read_to_string(f2) {
        Ok(s) => s,
        Err(e) => return Err(e)
    };
    This type of handling is so common that can be substituted as below
*/
    let s2 = fs::read_to_string(f2)?;
    s1.push('\n');
    s1.push_str(&s2);
    Ok(s1)
}

fn propagate_error_example() {
    let result = read_and_combine("files/planets.txt", "files/dwarf_planets.txt");
    match result {
        Ok(s) => println!("result is...\n{}", s),
        Err(e) => println!("There was an error: {}", e)
    };

}


fn error_challenge() {
    println!("Error challenge resolved");
/*     
    let secret_number = rand::rng().random_range(1..101);

    println!("I'm thinking of a number between 1 and 100...");
    println!("Guess the number:");
    loop {
        let mut buffer = String::new();
        let guess = match io::stdin().read_line(&mut buffer) {
            Ok(_) => match buffer.trim().parse::<u32>() {
                        Ok(value) => value, // success
                        Err(_) => {
                            println!("\nFailed to parse input. Guess again:");
                            continue
                        }
                     }
            Err(_) => {
                println!("\nFailed to read input. Guess again:");
                continue
            }
        };

        if guess > secret_number {
            println!("\n{} is too high! Guess lower:", guess);
        } else if guess < secret_number {
            println!("\n{} is too low! Guess higher:", guess);
        } else {
            println!("\nYou got it! The secret number was {}.", secret_number);
            break;
        }
    }    
*/
}