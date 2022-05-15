fn age_calculator(name: &str, current_age: u8) {
    let next_age = current_age + 1;
    println!("Hey {}, you will turn {} years old in your next birthday", name, next_age);
}

fn square(num: i32) -> i32 {
    num * num
}


fn main() {
    age_calculator("Nino", 32);
    age_calculator("Irene", 23);
    println!("4 squared is equal to {}", square(4));
    println!("5 squared is equal to {}", square(5));
}
