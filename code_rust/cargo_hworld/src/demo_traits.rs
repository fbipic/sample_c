use std::any;
use std::fmt; //Required to display item in print_type function

pub fn run(name: &str){
    println!("==>Welcome to demo_traits {}!!!", String::from(name));

    traits_example();
    default_traits_example();
    trait_bounds_example();
 }

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