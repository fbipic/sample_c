pub fn run(name: &str){
    println!("==>Welcome to demo_struct {}!!!", String::from(name));
    struct_example();
}

#[derive(Debug)] //Added to allow println macro to print structure in debug mode
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