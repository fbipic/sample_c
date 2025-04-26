use std::fs;
use std::io;

pub fn run(name: &str){
    println!("==>Welcome to demo_errors_handling {}!!!", String::from(name));

    unrecoverable_error_example();
    recoverable_error_example();
    match_result_example();
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