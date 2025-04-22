use std::mem; //Added in box example

pub fn run(name: &str){
    println!("==>Welcome to demo_gen_type {}!!!", String::from(name));
    gen_struct_example();
    gen_type_methods();
    gen_function_example();
    box_type_example();
}

#[derive(Debug)]    
struct Rectangle<T,U> {
    width: T,
    height: U
}

impl<T,U> Rectangle<T,U> {
    fn get_width(&self) -> &T {
        &self.width
    }
}

//This methods works only with u8
impl Rectangle<u8, u8> {
    fn get_perimeter(&self) ->u8 {
        2*self.width + 2*self.height
    }
}

fn gen_struct_example() {
    let rect = Rectangle {
        width: 1u8, 
        height: 3u16 //Change data type, so it is necessary to add U in definition
    };
    println!("rect is {:?}", rect);

    let rect2 = Rectangle {
        width: 1.2,
        height: 3.4
    };
    println!("rect2 is {:?}", rect2);

}

fn gen_type_methods(){
    let rect = Rectangle {
        width: 1u8, 
        height: 3u8
    };
    println!("rect is {:?}", rect);
    println!("rect width={}", rect.get_width());
    println!("rect perimeter={}", rect.get_perimeter());
}

fn get_biggest<T: PartialOrd>(a:T, b:T) -> T {
    //Using comparison with generic type it is not clear how to resolve it -> compiler error
    if a > b {
        a
    } else {
        b
    }
}

fn gen_function_example(){
    println!("biggest is {}", get_biggest(3.1, 5.1));
}

struct Shuttle {
    name: String,
    crew_size: u8,
    propellant: f64
}

fn box_type_example() {
    let vehicle = Shuttle {
        name: String::from("Atlantis"),
        crew_size: 7,
        propellant: 835958.0
    };
    println!("vehicle size on stack: {} bytes", mem::size_of_val(&vehicle));

    //After this point vehicle lose ownership
    let boxed_vehicle: Box<Shuttle> = Box::new(vehicle);
    //Here we get occupation of a pointer on stack
    println!("boxed_vehicle size on stack: {} bytes", mem::size_of_val(&boxed_vehicle));
    //Here we get occupation of structure on heap
    println!("boxed_vehicle size on stack: {} bytes", mem::size_of_val(&*boxed_vehicle));

    //Return ownership back to unboxed_vehicle
    let unboxed_vehicle: Shuttle = *boxed_vehicle;
    println!("unboxed_vehicle size on stack: {} bytes", mem::size_of_val(&unboxed_vehicle));
}