#[path = "./demo_data.rs"]
mod demo_data;

#[path = "./demo_function.rs"]
mod demo_function;

// Main program
fn main() {
    println!("Hello, RUST world with CARGO!");

    let user = "user";
    demo_data::run(user);

    demo_function::run(user);
}
