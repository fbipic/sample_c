pub fn run(name: &str){
    println!("==>Welcome to demo_struct {}!!!", String::from(name));
    struct_example();
    struct_expanded_example();
}

#[derive(Debug)] //Added to allow println macro to print structure in debug mode
#[derive(Clone)] //Added to allow clone operation in new data structure
struct Shuttle {
    name: String,
    crew_size: u8,
    propellant: f32
}

fn struct_example() {
    let mut vehicle = Shuttle{
        name: String::from("Endeavour"),
        crew_size: 7,
        propellant: 85697.0
    };
    println!("vehicle.name={}", vehicle.name);

    vehicle.name = String::from("Atlantis");
    println!("vehicle={:?}", vehicle);
}

fn struct_expanded_example() {
    let mut vehicle1 = Shuttle{
        name: String::from("Endeavour"),
        crew_size: 7,
        propellant: 85697.0
    };

    let vehicle2 = Shuttle{
        name: String::from("Discovery"), //String cannot be used because we lost ownership
        ..vehicle1 //copy value not declared explicitly from other definition
    };

    let vehicle3 = Shuttle {
        ..vehicle1.clone() //Used to clone all data including string
    };

    //Changes are affecting only first vehicle
    vehicle1.crew_size = 6;

    println!("vehicle1={:?}", vehicle1);
    println!("vehicle2={:?}", vehicle2);
    println!("vehicle3={:?}", vehicle3);
}