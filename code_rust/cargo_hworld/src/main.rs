fn print_float() {
    let mut x_64 = 10.12345678910123456; //64 bit float representation
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
    x_mut = 255;
    // x_mut = x_mut + 1; //Perform overflow error on runtime
}

// Main program
fn main() {
    println!("Hello, RUST world with CARGO!");
    print_integer();
    print_float();
}
