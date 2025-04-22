pub fn run(name: &str){
    println!("==>Welcome to demo_struct {}!!!", String::from(name));
    struct_example();
    struct_expanded_example();
    struct_method_example();
    associated_function_example();
    tuple_struct_example();

    struct_challenge();
}

#[derive(Debug)] //Added to allow println macro to print structure in debug mode
#[derive(Clone)] //Added to allow clone operation in new data structure
struct Shuttle {
    name: String,
    crew_size: u8,
    propellant: f64
}

impl Shuttle {
    fn get_name(&self) -> &str {
        &self.name
    }
    fn add_fuel(&mut self, gallons: f64){
      self.propellant += gallons;
    }
    //Associated function
    fn new(name: &str) -> Shuttle{
        Shuttle {
            name: String::from(name),
            crew_size : 7,
            propellant: 0.0
        }
    }
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

fn struct_method_example() {
    let mut vehicle = Shuttle{
        name: String::from("Endeavour"),
        crew_size: 7,
        propellant: 85697.0
    };
   
    let vehicle_name = vehicle.get_name();
    println!("vehicle_name={}", vehicle_name);

    println!("propellant={:.3}", vehicle.propellant);
    vehicle.add_fuel(1000.0);
    println!("propellant={:.3}", vehicle.propellant);
}

fn associated_function_example() {
    let vehicle1 = Shuttle::new("Endeavour");
    let vehicle2 = Shuttle::new("Discovery");

    let vehicle_name = vehicle1.get_name();
    println!("vehicle1_name={}", vehicle_name);

    let vehicle_name = vehicle2.get_name();
    println!("vehicle2_name={}", vehicle_name);
}

#[derive(Debug)] //Added to allow println macro to print structure in debug mode
struct Color(u8, u8, u8); //RGB
struct Point(u8, u8, u8); //XYZ

fn get_y(p: Point) -> u8 {
    p.1
}

fn tuple_struct_example() {
    let red = Color (255, 0, 0);
    println!("First value in red {}", red.0);
    println!("red={:?}", red);

    let coord = Point (3,4,5);
    let y = get_y(coord);
    println!("y={}", y);
}

struct Rectangle {
    width : f64,
    height: f64
}

impl Rectangle {
    fn get_area (&self) -> f64 {
        self.height * self.width
    }
    fn scale(&mut self, fact: f64) {
        self.height *= fact;
        self.width *= fact;
    }
    fn new(w: f64, h: f64) -> Rectangle {
        Rectangle { width: w, height: h }
    }
}

fn struct_challenge(){
    let mut rect = Rectangle::new(1.2, 3.4);
    assert_eq!(rect.get_area(), 4.08);
    rect.scale(0.5);
    assert_eq!(rect.get_area(), 1.02);
    println!("Tests passed!");
}