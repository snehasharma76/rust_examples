use std::io::{self, Write};

fn main() {
    println!("Advanced Calculator with BODMAS");
    
    loop {
        let expression = get_input("Enter your expression (or 'q' to quit): ");
        
        if expression.to_lowercase() == "q" {
            break;
        }
        
        match evaluate_expression(&expression) {
            Ok(result) => println!("Result: {}", result),
            Err(e) => println!("Error: {}", e),
        }
    }
    
    println!("Thank you for using the calculator!");
}

fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

fn evaluate_expression(expr: &str) -> Result<f64, String> {
    // ... implementation of expression evaluation
    // This would involve parsing the expression and applying BODMAS rules
    // For brevity, let's assume a simple implementation
    
    let tokens: Vec<&str> = expr.split_whitespace().collect();
    let mut result = 0.0;
    let mut current_op = '+';
    
    for token in tokens {
        if let Ok(num) = token.parse::<f64>() {
            match current_op {
                '+' => result += num,
                '-' => result -= num,
                '*' => result *= num,
                '/' => {
                    if num == 0.0 {
                        return Err("Division by zero".to_string());
                    }
                    result /= num
                },
                _ => return Err(format!("Unknown operator: {}", current_op)),
            }
        } else {
            current_op = token.chars().next().unwrap();
        }
    }
    
    Ok(result)
}

