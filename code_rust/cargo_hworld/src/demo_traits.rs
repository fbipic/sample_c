use std::any;
use std::fmt; //Required to display item in print_type function

pub fn run(name: &str){
    println!("==>Welcome to demo_traits {}!!!", String::from(name));

    traits_example();
    default_traits_example();
    derive_traits_example();
    trait_bounds_example();
    trait_multiple_bound_example();
    trait_return_type_example();
 }

//PartialEq allows us to derive == operator without implementing it
//PartialOrd allows us to derive > operator without implementing it
#[derive(PartialEq, PartialOrd)] 
 struct Satellite {
    name: String,
    velocity: f64 // miles per second
}

struct SpaceStation {
    name: String,
    crew_size: u8,
    altitude: u32 // miles
}

struct SpaceShuttle {
    name: String,
    crew_size: u8
}

trait Description {
//    fn describe(&self) -> String;
    //Defined a default traits that is valid for all formats
    fn describe(&self) -> String {
        String::from("an object flying through space!")
    }
}
impl Description for Satellite {
    fn describe(&self) -> String {
        format!("the {} flying at {} miles per second!", self.name, self.velocity)
    }
}
impl Description for SpaceStation {
    fn describe(&self) -> String {
        format!("the {} flying {} miles high with {} crew members aboard!", self.name, self.altitude, self.crew_size)
    }
}
impl Description for SpaceShuttle { 

}

fn traits_example() {
    let hubble = Satellite {
        name: String::from("Hubble Telescope"),
        velocity: 4.72
    };
    let iss = SpaceStation {
        name: String::from("International Space Station"),
        crew_size: 6,
        altitude: 254
    };
    println!("hubble is {}", hubble.describe());
    println!("iss is {}", iss.describe());
}

fn default_traits_example() {
    let ss = SpaceShuttle {
        name: String::from("Apollo 17"),
        crew_size: 3
    };
    //In this case we use default method description
    println!("ss is {}", ss.describe());
}

fn print_type<T: fmt::Display>(item: T) {
     println!("{} is {}", item, any::type_name::<T>());
}

fn print_type_debug<T: fmt::Debug>(item: T) {
    println!("{:?} is {}", item, any::type_name::<T>());
}

fn trait_bounds_example (){
    print_type(13);
    print_type(13.0);
    print_type("thirteen");
    //print_type([13]); //This is not implemented in display

    //Changing Display->Debug and output macro it will be display all type of input
    print_type_debug(13);
    print_type_debug(13.0);
    print_type_debug("thirteen");
    print_type_debug([13]);
}

fn derive_traits_example(){
    let hubble = Satellite {
        name: String::from("Hubble Telescope"),
        velocity: 4.72
    };
    let gps = Satellite {
        name: String::from("GPS"),
        velocity: 2.42
    };
    println!("hubble == gps is {}", hubble == gps);
    println!("hubble > gps is {}", hubble > gps);
}

//fn compare_and_print<T, U>(a: T, b: U) {
//fn compare_and_print<T: fmt::Display + PartialEq + From<U>, U: fmt::Display + PartialEq + Copy>(a: T, b: U) {
fn compare_and_print<T, U>(a: T, b: U) 
    where T: fmt::Display + PartialEq + From<U>, 
          U: fmt::Display + PartialEq + Copy
    {
    if a == T::from(b) {
        println!("{} is equal to {}", a, b);
    } else {
        println!("{} is NOT equal to {}", a, b);
    }
}

fn trait_multiple_bound_example() {
    compare_and_print(1.0, 1);
    compare_and_print(1.1, 1); 

    //compare_and_print(1.1, "one"); //String cannot be converted into number.
}

fn get_displayable() -> impl fmt::Display {
    //13
    "thirteen"
    //[13] //Fails to compile
}

fn trait_return_type_example() {
    println!("output is {}", get_displayable());
}