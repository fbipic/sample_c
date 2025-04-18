pub fn run(name: &str){
    println!("==>Welcome to demo_reference {}!!!", String::from(name));
    borrow_ownership();
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
