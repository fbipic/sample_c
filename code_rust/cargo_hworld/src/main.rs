#[path = "./demo_data.rs"]
mod demo_data;

// Main program
fn main() {
    println!("Hello, RUST world with CARGO!");

    let user = "Jose";
    demo_data::run(user);
}
