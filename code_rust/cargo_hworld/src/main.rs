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
    println!("a_f/b_f={:08.3}", a_f as f64/b_f);

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

fn print_bitwise() {
    let value= 0b1111_0101u8; //stored as unsigned int 8 bit
    println!("value = {} -> {0:08b}", value); //display 0 and 8 bit

    //Using NOT operator
    let mut val_m;
    val_m = !value;
    println!("val_n = {:3} -> {0:08b}", val_m);

    //Using OR operator
    val_m = val_m & 0b1111_0111;
    println!("val_a = {:3} -> {0:08b}", val_m);
    println!("bit 6 = {:3}", val_m & 0b0010_0000);

    //Using AND operator
    val_m = val_m | 0b0100_0000;
    println!("val_o = {:3} -> {0:08b}", val_m);

    //Using XOR operator
    val_m = val_m ^ 0b0101_0101;
    println!("val_o = {:3} -> {0:08b}", val_m);

    //Using shift 
    val_m = val_m << 4;
    println!("val_l = {:3} -> {0:08b}", val_m);

    val_m = val_m >> 2;
    println!("val_r = {:3} -> {0:08b}", val_m);

}

fn print_boolean () {
    let a = true;
    let b = false;
    println!("a is {} and b is {}", a, b);
    println!("NOT a is {}", !a);
    println!("a AND b is {}", a & b);
    println!("a OR b is {}", a | b);
    println!("a XOR b {}", a ^ b);

    let mut c = (a ^ b) | (a & b);
    println!("(a ^ b) | (a & b) = {} ", c);

    c = (a ^ b) || (a & b); //Ignore=optimization right side if first expression is true
    println!("(a ^ b) || (a & b) = {} ", c);
    c = (a ^ b) || panic!(); //macro that cause program to exit immediately
    println!("(a ^ b) || (a & b) = {} ", c);
    //c = (a ^ b) && panic!(); //Need to evaluate also second part then crash    
}

fn print_comparison (){
    //let a = 2;
    //let b = 1;
    //Works also for boolean
    let a = true;
    let b = false;
    //Cannot be using different data types
    println!("a={} b={}", a, b);
    println!("a = b = {}", a==b);
    println!("a > b = {}", a>b);
    println!("a < b = {}", a<b);
}

fn print_chars() {
    //Char is stored in 4 bytes as unicode
    let letter = 'a';
    let number  = '1';
    let finger = '\u{261D}';
    println!("{}\t{}\t{}", letter, number, finger);
}

fn print_average() {
    let a = 13;
    let b = 2.3;
    let c: f32 = 120.0;

    //Challenge. If used f32 precision is not enough to satisty assert
    let average = (a as f64 + b + c as f64) / 3 as f64;

    assert_eq!(average, 45.1);
    println!("Test passed")
}

// Main program
fn main() {
    println!("Hello, RUST world with CARGO!");
    print_integer();
    print_float();
    print_operation();
    print_bitwise();
    print_boolean();
    print_comparison();
    print_chars();

    //Challenge
    print_average();
}
