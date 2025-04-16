pub fn run(name: &str){
    println!("==>Welcome to demo_flow {}!!!", String::from(name));

    conditional_flow(true);
    conditional_flow(false);
    conditional_nested_flow(3, 5);
    conditional_assignment(true);

    flow_loop();
    flow_while();
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

fn conditional_assignment(condition:bool) {
/*    
    let x;
    if condition {
        x = 1;
    } else {
        //if we know that condition is always true, compiler terminate with
        //error if we comment out this line
        x = 2;
    }
*/
    //Following line can be used to substitute lines above, data type of assignment should be same
    let x = if condition {1} else {2};
    println!("x={} because condition={}", x, condition);
}

fn flow_loop(){
    let mut counter = 0;
    println!("loop start");
    //Loop execute a code in an infinite cycle
    //In this case is used as an expression
    loop {
        counter += 1;
        println!("counter = {}", counter);
        if counter == 10 { 
            //This instruction allow to exit from loop immediately
            break; 
        };
    }
    println!("loop exit");

    //We can exit from loop returning a value
    //In this case with let, loop became a statement and required ;
    counter = 0;
    let result = loop {

        counter += 1;
        if counter == 5 {
            //Loop can return a value in break statement
            break counter;
        }
    };
    println!("loop exit with result = {}", result);

}

fn flow_while() {
    let mut count = 0;
    while count < 10{
        count += 1;
        print!("{} ", count);
    }
    println!("I come");

    let letters = ['a', 'b', 'c'];
    count = 0;
    while count < letters.len(){
        println!("letter={} ", letters[count]);
        count += 1;

    }

}
