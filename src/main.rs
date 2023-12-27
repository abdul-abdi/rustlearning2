use std::io;
fn main() {
    println!("Input the first number: ");
    let mut a = String::new();
    io::stdin().read_line(&mut a).expect("Failed to read value a");
    let first_number: f64 = a.trim().parse().expect("Invalid first number");

    println!("Input the operation to be performed (+, -, *, /): ");
    let mut operand = String::new();
    io::stdin().read_line(&mut operand).expect("Failed to read operand");
    let operation = operand.trim().to_uppercase();

    println!("Input the second number: ");
    let mut b = String::new();
    io::stdin().read_line(&mut b).expect("Failed to read value b");
    let second_number:f64 = b.trim().parse().expect("Invalid Second NUmber");

    let operation_instance = match operation.as_str() {
        "+" => Operation::Add(first_number, second_number),
        "-" => Operation::Subtract(first_number, second_number),
        "*" => Operation::Multiply(first_number, second_number),
        "/" => Operation::Divide(first_number, second_number),
        _ => panic!("Invalid operation"),
    };

    let end_result = calculate(operation_instance);

    println!("The answer to your problem is {}", end_result);
}

enum Operation {
    Add (f64, f64),
    Subtract (f64, f64),
    Multiply(f64, f64),
    Divide (f64, f64),
}

fn calculate (x : Operation) -> f64 {
    match x {
        Operation::Add(a, b ) => a + b,
        Operation::Subtract(a,b ) => a - b,
        Operation::Multiply(a,b ) => a * b,
        Operation::Divide(a,b ) => a / b,
    }
}