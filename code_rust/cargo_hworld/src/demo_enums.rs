pub fn run(name: &str){
    println!("==>Welcome to demo_enums {}!!!", String::from(name));
    define_enum_example();
    match_operator_example();
    match_with_default_example();
    enum_methods_example();
    option_t_example();
    matching_t_example();
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

impl Shape {
    fn get_perimeter(&self) -> f64 {
        match *self {
            Shape::Circle(r) => r * 2.0 * std::f64::consts::PI,
            Shape::Triangle(a, b, c) => a+b+c,
            Shape::Rectangle(w, h ) => 2.0*w + 2.0*h
        }
    }
}

fn enum_methods_example() {
    let my_shape = Shape::Rectangle(1.2, 3.4);
    println!("my_shape = {:?}", my_shape);
    println!("my_shape perimeter = {}", my_shape.get_perimeter());
}

fn option_t_example() {
    let countdown = [5, 4, 3, 2, 1];
    //let number = countdown[5]; //Index is out of bounds
    let number = countdown.get(5); //Returns Option enum None
    let number = number.unwrap_or(&0) +1; //Added unwrap_or to return value if option is None
    println!("number is {:?}", number);

    let number = countdown.get(1); //Returns Option enum Some
    let number = number.unwrap() +1; //Added unwrap because type is different, but it does not work in case of None
    println!("number is {:?}", number); 

}

fn matching_t_example() {
    let countdown = [5, 4, 3, 2, 1];   
    let number = countdown.get(5);
    let number = match number {
        Some(number) => number +1,
        None => 0
    };
    println!("number is {:?}", number);

}