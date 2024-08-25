use std::io;


fn main() {
    println!("Start of the calculator program");

    println!("Let's implement the basic functions addition, subtraction, division and multiplication first with two inputs");

    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut oper = String::new();
    let result = i32;

    println!("Please input the first number");
    io::stdin().read_line(&mut input1).expect("Invalid input");

    println!("Let's input the second number");
    io::stdin().read_line(&mut input2).expect("Invalid input");

    println!("List of operations to be performed");
    println!("1. Addition");
    println!("(2) Subtraction");
    println!("(3) Multplication");
    println!("(4) Division");
    println!("Select the oepration to be performed!");
    io::stdin().read_line(&mut oper).expect("Invalid input");

    //need to figure out why is it not working as string here 
    match oper {
        1 => result = input1 + &input2,
        2 => result = input1 - &input2,
        3 => result = input1 * &input2,
        4 => result = input1 / &input2,
        - => {
            println!( "Invalid Selection");
            return;
        }
    }
    println!("The result is: {}", result);
    //fn addition(input1 | input2){
        
    }
    

