use std::io; //Declaration to use i/o modules
//use rand; //Use random library available from crates.io
use rand::{prelude::*, thread_rng}; //Load all prelude functions

pub fn run(name: &str){
    println!("==>Welcome to demo_reference {}!!!", String::from(name));
    io_example();
    rand_example();
}

fn io_example() {
    let mut buffer = String::new();
    println!("Enter a message");
    let err= io::stdin().read_line(&mut buffer);
    println!("buffer={}\nerr={:?}", buffer, err);

    //Convert input into number that can be used to process data
    //let number = buffer.trim().parse::<i32>(); //to be checked 
    let number:i32 = buffer.trim().parse().unwrap();
    println!("number + 1 = {}", number+1);
}

fn rand_example()
{
    let number= rand::random::<f64>();
    println!("Random number = {:.3}", number);

    let number = thread_rng().gen_range(1..11);
    println!("Generate in range = {:.3}", number);
}