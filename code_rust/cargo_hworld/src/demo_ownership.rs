pub fn run(name: &str){
    println!("==>Welcome to demo_ownership {}!!!", String::from(name));

    scope_example();
    shadow_example();
}

//Variable lives into curly brackets 
fn scope_example() {
    if true {
        let planet = "Earth";
        println!("Planet is {}", planet);
    }
    //println!("Planet is {}", planet); //This cause error during build because variable is out of scope
}

fn shadow_example () {
    let planet = "Earth";
    {
        println!("Planet is {}", planet);
        let mut planet = "4";
        println!("Planet is {}", planet);
    }
    //Shadow disappear after curly brackets
    println!("Planet is {}", planet);
}