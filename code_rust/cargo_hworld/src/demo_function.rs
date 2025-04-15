pub fn run(name: &str) {
    println!("==>Welcome to demo_function {}!!!", String::from(name));

    let x= 1;
    let y= 2;
    println!("sum {}+{}={}", x, y, sum_value(x,y));
    println!("square of y^2={}", square_value(y));
    let square = square_tuple(y);
    println!("result is {:?}", square); //Used to display multiple values
    println!("square of y({})^2={}", square.0, square.1);

    function_challenge();
}

//Rust does not care if function order in file
fn sum_value(a:u8, b:u8) -> u8{
    //a+b is an expression
    //a+b; is a statement due to ;
    let sum: u8 = a+b;
    return sum;
    //Advise that after this point instruction are not executed
    //println!("End sum");
}

fn square_value(x: u8) -> u8{
    //If last instruction is an expression, rust use it as return value
    x * x
}

fn square_tuple(x: u8) -> (u8, u8){
    (x, x*x)
}

fn celsius_to_fahrenheit(celsius: f64)-> f64 {
    (1.8 * celsius) + 32.0
}

fn function_challenge(){
    let celsius_temp : f64 = 23.0;
    let fahrenheit_temp = celsius_to_fahrenheit(celsius_temp);
    println!("C°={} -> F°={}", celsius_temp, fahrenheit_temp);

    assert_eq!(fahrenheit_temp, 73.4);
    println!("Test passed")
}