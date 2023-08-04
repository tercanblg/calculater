use std::io;

enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

fn calculate(op: Operation) -> f64 {
    match op {
        Operation::Add(a, b) => a + b,
        Operation::Subtract(a, b) => a - b,
        Operation::Multiply(a, b) => a * b,
        Operation::Divide(a, b) => a / b,
    }
}

fn main() {
    println!("Simple Calculator");
    println!("------------------");

    println!("Enter the first number:");
    let mut num1_input = String::new();







    io::stdin().read_line(&mut num1_input).expect("Failed to read input.");
    let num1: f64 = num1_input.trim().parse().expect("Invalid number entered.");

    println!("Enter the operation (+, -, *, /):");
    let mut operator_input = String::new();
    io::stdin().read_line(&mut operator_input).expect("Failed to read input.");
    let operator: char = operator_input.trim().chars().next().expect("Invalid operator entered.");

    println!("Enter the second number:");
    let mut num2_input = String::new();
    io::stdin().read_line(&mut num2_input).expect("Failed to read input.");
    let num2: f64 = num2_input.trim().parse().expect("Invalid number entered.");

    let operation = match operator {
        '+' => Operation::Add(num1, num2),
        '-' => Operation::Subtract(num1, num2),
        '*' => Operation::Multiply(num1, num2),
        '/' => Operation::Divide(num1, num2),
        _ => panic!("Invalid operator entered."),
    };

    let result = calculate(operation);
    println!("Result: {}", result);
}