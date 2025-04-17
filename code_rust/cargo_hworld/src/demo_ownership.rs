pub fn run(name: &str){
    println!("==>Welcome to demo_ownership {}!!!", String::from(name));

    scope();
}

//Variable lives into curly brackets 
fn scope() {
    if true {
        let planet = "Earth";
        println!("Planet is {}", planet);
    }
    //println!("Planet is {}", planet); //This cause error during build because variable is out of scope
}