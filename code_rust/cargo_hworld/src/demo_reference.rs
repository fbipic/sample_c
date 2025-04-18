pub fn run(name: &str){
    println!("==>Welcome to demo_reference {}!!!", String::from(name));
    borrow_ownership();
    borrow_mut_ownership();
}

fn borrow_ownership(){
    let rocket_fuel = String::from("RP-1");
    let (rocket_fuel, length) = processing_fuel(rocket_fuel);
    println!("rocket_fuel[{}] = {}", length, rocket_fuel);
    //Passing value using borrow operator & and avoid to transfer ownership
    let length = processing_fuel_len(&rocket_fuel); 
    println!("rocket_fuel[{}] = {}", length, rocket_fuel);
}

fn processing_fuel(propeller: String) -> (String, usize){
    println!("propeller = {}", propeller);
    let length = propeller.len();
    (propeller, length)
}

fn processing_fuel_len(propeller: &String) -> usize{
    println!("propeller = {}", propeller);
    let length = propeller.len();
    length
}

fn borrow_mut_ownership(){
    let mut rocket_fuel = String::from("RP-1");
    let length = processing_fuel_mut(&mut rocket_fuel); 
    println!("rocket_fuel[{}] = {}", length, rocket_fuel);
}

fn processing_fuel_mut(propeller: &mut String) -> usize {
    //Restriction: only a mutable reference can be created
    println!("propeller = {}", propeller);
    propeller.push_str(" is highly flammable!");
    let length = propeller.len();
    length
}
