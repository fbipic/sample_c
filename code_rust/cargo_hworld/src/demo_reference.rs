pub fn run(name: &str){
    println!("==>Welcome to demo_reference {}!!!", String::from(name));
    borrow_ownership();
    borrow_mut_ownership();
    dangling_reference_example();
    slice_example();
    slice_parameter_example();

    borrow_and_slice_challenge();
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

fn dangling_reference_example(){
    let rocket_fuel = produce_fuel();
    println!("rocket_fuel = {}", rocket_fuel);  
}

/* 
fn produce_fuel () -> &String {
    //Data and stack are no longer available after function is executed and this cause an lifetime error
    let new_fuel = String::from("RP-1");
    &new_fuel
}
*/
fn produce_fuel () -> String {
    //Issue in dangling can be resolved passing ownership
    let new_fuel = String::from("RP-1");
    new_fuel
}

fn slice_example() {
    //slice= contiguous section of a collection, without taking ownership
    let message = String::from("Greetings from Earth");
    println!("message={}", message);

    //let last_word = &message[15..15+5]; //Index of earth + length
    //let last_word = &message[15..15+50]; //Cause crash on runtime
    let last_word = &message[15..]; //Index of earth until end of string
    //Char is in utf-8 and it can occupy multiple byte, so can be dangerous to use is as it is
    println!("last_world={}", last_word);

    let planets = [1,2,3,4,5,6,7,8,9];
    let inner_planet: &[i32] = &planets[..4];
    println!("inner_planet = {:?}", inner_planet);

}

fn slice_parameter_example(){
    let message = String::from("Greetings from Earth");
    println!("message={}", message);

    let first_word = get_first_word(&message);
    println!("first_word={}", first_word);

    let next_word = get_word(&message[10..]);
    println!("next_word={}", next_word);

    //String is converted into slice = deref coercion 
    let next_word = get_word(&message);
    println!("next_word={}", next_word);
}

//&String = string reference != &str slice string
//String can be used as slice reference but not vice-versa
fn get_first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..index]; //Space found
        }
    }
    &s //No space found
}

fn get_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..index]; //Space found
        }
    }
    &s //No space found
}

fn borrow_and_slice_challenge() {
    let test1 = "We need more space.";
    assert_eq!(trim_spaces(test1), "We need more space.");
    
    let test2 = String::from("   There's space in front.");
    assert_eq!(trim_spaces(&test2), "There's space in front.");
    
    let test3 = String::from("There's space to the rear. ");
    assert_eq!(trim_spaces(&test3[..]), "There's space to the rear.");   
    
    let test4 = "  We're surrounded by space!    ";
    assert_eq!(trim_spaces(test4), "We're surrounded by space!");
    
    let test5 = "     ";
    assert_eq!(trim_spaces(test5), "");
    
    let test6 = "";
    assert_eq!(trim_spaces(test6), "");
    
    let test7 = " 🚀 ";
    assert_eq!(trim_spaces(test7), "🚀");
    println!("Tests passed!");
}

fn trim_spaces(s: &str) -> &str {
    let mut char_first = 0;
    let mut char_last = 0;
    for (index, character) in s.chars().enumerate() {
        if character != ' ' { 
            char_first = index;
            break;
        }
    }
    for (index, character) in s.chars().rev().enumerate() {
        if character != ' ' { 
            char_last = s.len()-index;
            break;
        }
    }

    println!("char_first={} char_last={} len={}", char_first, char_last, s.len());
    &s[char_first..char_last]
}