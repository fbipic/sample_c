pub fn run(name: &str){
    println!("==>Welcome to demo_enums {}!!!", String::from(name));
    define_enum_example();
    match_operator_example();
    match_with_default_example();
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

fn match_operator_example(){
    let my_shape = Shape::Rectangle(1.2, 3.4);
    println!("my_shape = {:?}", my_shape);

    match my_shape {
        Shape::Circle(r) => println!("This is a circle with radius {}", r),
        Shape::Rectangle(w, h) => println!("This is a rectangle with width {} and hight {}", w, h),
        Shape::Triangle(a,b ,c ) => println!("This is a triangle with sides {} {} {}", a, b, c)
    } 
}

fn match_with_default_example() {
    let my_number = 1u8;

    let result = match my_number {
        0 => "zero",
        1 => "one",
        2 => "two",
        3 => "three",
        //Wildcard pattern should be always at the end
        _ => { println!("{} did not match", my_number);
                "something else"
        }
    };
    println!("result = {}", result);
}