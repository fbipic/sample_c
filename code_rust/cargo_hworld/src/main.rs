#[path = "./demo_data.rs"]
mod demo_data;

#[path = "./demo_function.rs"]
mod demo_function;

#[path = "./demo_flow.rs"]
mod demo_flow;

#[path = "./demo_ownership.rs"]
mod demo_ownership;

#[path = "./demo_reference.rs"]
mod demo_reference;

// Main program
fn main() {
    println!("Hello, RUST world with CARGO!");

    let user = "user";
    demo_data::run(user);

    demo_function::run(user);
    demo_flow::run(user);
    demo_ownership::run(user);
    demo_reference::run(user);
}
