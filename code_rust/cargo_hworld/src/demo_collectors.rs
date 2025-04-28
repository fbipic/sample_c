pub fn run(name: &str){
    println!("==>Welcome to demo_collectors {}!!!", String::from(name));

    vector_example();
}

fn vector_example(){
    let mut astronauts: Vec<String> = Vec::new();
    astronauts.push(String::from("Shepard")); // Alan Shepard
    astronauts.push(String::from("Grissom")); // Gus Grissom
    astronauts.push(String::from("Glenn")); // John Glenn
    println!("astronauts is {:?}", astronauts);

    let last = astronauts.pop();
    println!("last is {:?}", last);

    //We need to use borrow operator because we cannot pass string ownership to variable
    //let third = &astronauts[2]; //Adding code above remove last element use get instead
    //println!("third astronaut is {}", third);
    let third = astronauts.get(2); //return enum option
    println!("third astronaut is {:?}", third);

    //Macro to define vector
    let countdown = vec![5,4,3,2,1,0];
    println!("countdown = {:?}", countdown);
}