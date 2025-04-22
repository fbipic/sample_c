//use std::env; //Include to use arguments

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

#[path = "./demo_modules.rs"]
mod demo_modules;

#[path = "./demo_io.rs"]
mod demo_io;

#[path = "./demo_struct.rs"]
mod demo_struct;

#[path = "./demo_gen_type.rs"]
mod demo_gen_type;

#[path = "./demo_traits.rs"]
mod demo_traits;

// Main program
fn main() {
    println!("Hello, RUST world with CARGO!");

/* 
    for (index, arg) in env::args().enumerate() {
        println!("arg[{}] = {}", index, arg);
    }
*/
    let user = "user";
    demo_data::run(user);

    demo_function::run(user);
    demo_flow::run(user);
    demo_ownership::run(user);
    demo_reference::run(user);
    demo_modules::run(user);
    demo_io::run(user);
    demo_struct::run(user);
    demo_gen_type::run(user);
    demo_traits::run(user);
}
