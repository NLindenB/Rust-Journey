fn printing_value() {
    let x = 2;
    let y = 3;
    let z = x + y;
    println!("My dog is {} years old", z);
}

fn returning_value() -> i32{
    let x: i32 = 32;
    return x;
}

fn returning_value_with_variable() -> i32 {
    33
}

fn main() {
    printing_value();
    let age = returning_value();
    let next_age = returning_value_with_variable();

    println!("I am {} years old", age);
    println!("Next year, I will be {} years old", next_age);
}
