pub fn run(name: &str){
    println!("==>Welcome to demo_lifetime {}!!!", String::from(name));

    borrow_checker_example();
    lifetime_annotation_syntax_example();
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

//fn best_fuel(x: &str, y: &str) -> &str { //Error because expected named lifetime parameter
//This syntax tell how manage lifetime by compiler
fn best_fuel<'a>(x: &'a str, y: &'a str) -> &'a str  {  
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn lifetime_annotation_syntax_example() {
    let result;
    let propellant1 = String::from("RP-1");
    let propellant2 = String::from("LNG");
    result = best_fuel(&propellant1, &propellant2);
    println!("result is {}", result);

    let result2;
    {
        let propellant3 = String::from("LNG");
        result2 = best_fuel(&propellant1, &propellant3); //THis fails
    }
    println!("result2 is {}", result2);
}