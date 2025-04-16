pub fn run(name: &str){
    println!("==>Welcome to demo_flow {}!!!", String::from(name));

    conditional_flow(true);
    conditional_flow(false);
    conditional_nested_flow(3, 5);
}

fn conditional_flow(x: bool){
    if x {
        println!("conditional_flow x=true");
    }
    else {
        println!("conditional flow x=false");
    }
}

fn conditional_nested_flow(x:u8, y:u8){
    if x > y {
        println!("x={} is greater than y={}", x, y);
    }
/* 
    These lines can be substituted with else if statement as below    
    else {
        if x < y {
            println!("x={} is less than y={}", x, y);
        }
        else {
            println!("x={} is equal to y={}", x, y);
        }
    }
*/
    else if x < y {
        println!("x={} is less than y={}", x, y);
    }
    else {
        println!("x={} is equal to y={}", x, y);
    }
}