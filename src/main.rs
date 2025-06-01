use std::io;

fn main() {

    let mut input = String::new();

    println!("Enter the first number:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let a: f64 = input.trim().parse().expect("Please enter a valid number");
    input.clear();

    println!("Enter the operation (+,-,*,/):");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let operator = input.trim().to_string();
    input.clear();

    println!("Enter the second number:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let b: f64 = input.trim().parse().expect("Please enter a valid number");

    let operation = match operator.as_str(){
        "+" => Operation::Add(a,b),
        "-" => Operation::Subtract(a,b),
        "*" => Operation::Multiply(a,b),
        "/" => Operation::Divide(a,b),
        _ => {
            println!("Invalid operator.");
            return;

        }
    };

    match calculate(operation) {
        Ok(result) => println!("Result: {}",result),
        Err(e) => println!("Error: {}",e),
    }




}

enum Operation {
    Add(f64,f64),
    Subtract(f64,f64),
    Multiply(f64,f64),
    Divide(f64,f64),
}

fn calculate(op: Operation) -> Result<f64, String> {
    match op {
    Operation::Add(a, b) => Ok(a + b),
    Operation::Subtract(a, b) => Ok(a - b),
    Operation::Multiply(a, b) => Ok(a * b),
    Operation::Divide(a, b) => {
        if b == 0.0 {
            Err("calculation is not possible. ".to_string())
        } else {
            Ok(a / b)
        }
    }
}}