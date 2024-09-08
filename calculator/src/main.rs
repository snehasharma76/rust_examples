use std::io;


fn main() {
    println!("Start of the calculator program");

    println!("Let's implement the basic functions addition, subtraction, division and multiplication first with two inputs");

    let mut input1:String = String::new();
    let mut input2: String = String::new();
    let mut oper: String = String::new();
    let result: i32;

    println!("Please input the first number");
    io::stdin().read_line(&mut input1).expect("Invalid input");
    let input1: i32 = match input1.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input!");
            return;
        }
    };


    println!("Let's input the second number");
    io::stdin().read_line(&mut input2).expect("Invalid input");
    let input2: i32 = match input2.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input!");
            return;
        }
    };

    println!("List of operators:");
    println!("(1) Add");
    println!("(2) Subtract");
    println!("(3) Multiply");
    println!("(4) Divide");
    println!("Select the number associated with the desired operation: ");

    io::stdin().read_line(&mut oper).expect("Invalid Input");
    let oper: i32 = match oper.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input!");
            return;
        }
    };

    match oper {
        1 => result = input1 + input2,
        2 => result = input1 - input2,
        3 => result = input1 * input2,
        4 => result = input1 / input2,
        _ => {
            println!("Invalid selection");
            return;
        }
    }

    println!("The result is: {}", result);
}

   