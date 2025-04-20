use std::io; //Declaration to use i/o modules

pub fn run(name: &str){
    println!("==>Welcome to demo_reference {}!!!", String::from(name));
    io_example();

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