pub fn run(name: &str){
    println!("==>Welcome to demo_ownership {}!!!", String::from(name));

    scope_example();
    shadow_example();
    string_example();
    ownership_example();
    transfer_ownership_example();
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
        let planet = "4";
        println!("Planet is {}", planet);
    }
    //Shadow disappear after curly brackets
    println!("Planet is {}", planet);
}

//Stack are stored in LIFO order, very quickly to access, limited in size, size is constant
//Heap access is lower than stack, dynamically added and removed

/* 
"String" string literal, hardcoded in code, immutable, known before compile
         string type, allocated on head, mutable
*/

fn string_example(){
    let mut message = String::from("Earth");
    println!("{}", message);
    message.push_str(" is our home.");
    println!("{}", message);
}

//Rules
//1. Every value is owned by one and only one variable at a time
//2. When variable goes out of scope, the value is dropped.

fn ownership_example(){
    {
        let inner_planet = "Earth";
        println!("inner_planet is {}", inner_planet);
    }
    //println!("Planet is {}", inner_planet);//Variable does not exist anymore and cause error

    let outer_planet: String;
    {
        let mut inner_planet = String::from("in_Earth");
        println!("inner_planet is {}", inner_planet);
        //Here we have two pointer to same heap memory, this means in "a move"
        //so outer_planet receives ownership of variable, because inner_planet is invalidated
        outer_planet = inner_planet;
        //println!("inner_planet is {}", inner_planet); //Compiler returns error because inner_ is not valid 
        inner_planet = outer_planet.clone(); //Duplicate memory area
        println!("inner_planet cloned is {}", inner_planet);
    }
    println!("outer_planet is {}", outer_planet);

}

fn transfer_ownership_example()
{
    let rocket_fuel = 1;
    processing_fuel(rocket_fuel);
    println!("rocket_fuel = {}", rocket_fuel);

    let rocket_fuel_s = String::from("RP-1");
    //Should add clone here otherwise ownership is transferred to propeller_s variable
    //and this will be destroyed and also data after function execution
    processing_fuel_s(rocket_fuel_s.clone());
    println!("rocket_fuel_s = {}", rocket_fuel_s);

    //Use same string also in function returning a value
    //Use shadow to overwrite previous variable 
    let rocket_fuel_s = processing_fuel_r(rocket_fuel_s);
    println!("rocket_fuel_s = {}", rocket_fuel_s);
}

//propeller is a stack copy of initial value
fn processing_fuel(mut propeller: i32) {
    propeller += 1;
    println!("propeller = {}", propeller);
}

fn processing_fuel_s(propeller_s: String) {
    println!("propeller = {}", propeller_s);
}

fn processing_fuel_r(propeller_s: String) -> String{
    println!("propeller = {}", propeller_s);
    propeller_s
}