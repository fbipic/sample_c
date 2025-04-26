pub fn run(name: &str){
    println!("==>Welcome to demo_errors_handling {}!!!", String::from(name));

    unrecoverable_error_example();
}

//Recoverable errors -> Result<T,E>
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