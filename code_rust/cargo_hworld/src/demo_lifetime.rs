pub fn run(name: &str){
    println!("==>Welcome to demo_lifetime {}!!!", String::from(name));

    borrow_checker_example();
    lifetime_annotation_syntax_example();
    multiple_lifetime_annotation_example();
    lifetime_elision_rules_example();
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

    //This example will fail
/*     
    let result2;
    {
        let propellant3 = String::from("LNG");
        result2 = best_fuel(&propellant1, &propellant3);
    }
    println!("result2 is {}", result2);
*/
}

//If we return always x, y lifetime should be different
//We can remove it from previous definition but it cause some question or add another, as done below
fn best_fuel_ax<'a, 'b>(x: &'a str, y: &'b str) -> &'a str  {  
    if x.len() > y.len() {
        x
    } else {
        x
    }
}


fn multiple_lifetime_annotation_example() {
    let result;
    let propellant1 = String::from("RP-1");
    {
        let propellant2 = String::from("LNG");
        result = best_fuel_ax(&propellant1, &propellant2);
    
    println!("result is {}", result);
    }
}

fn lifetime_elision_rules_example() {
    let message = String::from("Greetings from Earth!");
    let first_word = get_first_word(&message);
    println!("first_word is {}", first_word);
}

/*
This declaration with lifetime was not necessary anymore due to lifetime elision rules:
1. Each input that is a reference it is assigned an own lifetime
2. If there is exactly one input lifetime, assign it to all output lifetime
3. If there is a &self or &mut self input parameter, its lifetime will be assigned to all output lifetimes
Required annotation when ambiguity
*/
//fn get_first_word<'a>(s: &'a str) -> &'a str { 
fn get_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..index]; // found a space!
        }
    }

    &s // no spaces found; input is a single word
}