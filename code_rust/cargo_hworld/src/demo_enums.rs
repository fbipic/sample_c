pub fn run(name: &str){
    println!("==>Welcome to demo_enums {}!!!", String::from(name));
    define_enum_example();
}

#[derive(Debug)]
enum Shape {
    Circle(f64), //Can specify parameter with unknown name for every element of enum
    Triangle(f64, f64, f64), 
    Rectangle(f64, f64)
}

fn define_enum_example() {
    let my_shape = Shape::Rectangle(1.2, 3.4);
    println!("my_shape = {:?}", my_shape);
}