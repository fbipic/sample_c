pub fn run(name: &str){
    println!("==>Welcome to demo_gen_type {}!!!", String::from(name));
    gen_struct_example();
}

#[derive(Debug)]    
struct Rectangle<T,U> {
    width: T,
    height: U
}

fn gen_struct_example() {
    let rect = Rectangle {
        width: 1u8, 
        height: 3u16 //Change data type, so it is necessary to add U in definition
    };
    println!("rect is {:?}", rect);

    let rect2 = Rectangle {
        width: 1.2,
        height: 3.4
    };
    println!("rect2 is {:?}", rect2);

}