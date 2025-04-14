fn print_operation() {
    let a = 10;
    let b = 3;
    println!("a+b={}", a+b);
    println!("a-b={}", a-b);
    println!("a*b={}", a*b);
    println!("a/b={} a%b={}", a/b, a%b);

    //Use float and compare with latest one
    let a_f = 10;
    let b_f = 3.0;
    // Use as for casting integer into fload otherwise operation will fail
    println!("a_f/b_f={} a_f%b_f1={}", a_f as f64 /b_f, a_f as f64 %b_f);

    //Display with precision decided 
    // after with .3
    // before with 8 spaces
    println!("b_f={:8.3}", b_f);

    print!("a_f={} b_f={} ",a_f, b_f); //Print without carriage return
    // Display with 0 in front
    println!("a_f/b_f={:08.3}", a_f/b_f);

    //Using positinal index to print multiple time same variables
    println!("a ={0} b={1} => {0}/{1}={2:.3}", a, b, (a as f64/ b as f64));
}

fn print_float() {
    let x_64 = 10.12345678910123456; //64 bit float representation
    let x_32: f32 = 0.12345678910123456;
    println!("x_64 = {} x_32={}", x_64, x_32);
}

fn print_integer() {
    let x_imm : u8 = 17;     //Default as immutable
    let mut x_mut : u8 = 25; //Defined as mutable
    println!("x_imm = {} x_mut={}", x_imm, x_mut);
    /* x = 20; fails because variable is immutable */
    x_mut = 3;
    /* x_mut = -3; fails because variable is declared as unsigned */
    /* x_mut = 1000; fails because variable not in u8 range */
    println!("x_mut = {}", x_mut);
    //x_mut = 255;
    //x_mut = x_mut + 1; //Perform overflow error on runtime
}

// Main program
fn main() {
    println!("Hello, RUST world with CARGO!");
    print_integer();
    print_float();
    print_operation();
}
