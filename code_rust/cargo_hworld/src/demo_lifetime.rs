pub fn run(name: &str){
    println!("==>Welcome to demo_lifetime {}!!!", String::from(name));

    borrow_checker_example();
}

fn borrow_checker_example () {
    let propellant;
    let rp1 = String::from("RP-1");
    {
        //let rp1 = String::from("RP-1"); //Commented out to solve issue with print
        propellant = &rp1;
        println!("propellant is {}", propellant);
    }
    //propellant is not defined after this point
    //This cause an error, rp1 is no longer available
    //Only possible solution is to move rp1 declaration before curly brackets
    println!("propellant is {}", propellant); 
}