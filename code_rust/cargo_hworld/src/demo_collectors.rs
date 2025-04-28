use std::collections::HashMap; //Required to be added

pub fn run(name: &str){
    println!("==>Welcome to demo_collectors {}!!!", String::from(name));

    vector_example();
    hash_map_example();
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

fn hash_map_example() {
    let mut missions_flown = HashMap::new(); // missions flown as of 1 Jan 2021
    missions_flown.insert("Hadfield", 3); // Chris Hadfield
    missions_flown.insert("Hurley", 3); // Doug Hurley
    missions_flown.insert("Barron", 0); // Kayla Barron

    //Manage hash map
    missions_flown.insert("Barron", 1); //Override existing key-value pair
    //Insert new entry if does not exist
    missions_flown.entry("Barron").or_insert(2); //Key already exist, so this does not change value
    missions_flown.entry("Stone").or_insert(2); //New value pair will be inserted
    //Modify value based on its existing value
    let kayla = missions_flown.entry("Barron").or_insert(0); //This return reference
    *kayla += 1;
    println!("missions_flown is {:?}", missions_flown);

    let barron_mission = missions_flown.get("Barron");
    println!("Barron missions are {:?}", barron_mission);

    

}