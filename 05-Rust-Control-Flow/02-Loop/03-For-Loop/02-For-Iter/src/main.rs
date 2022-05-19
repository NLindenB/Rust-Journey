fn for_iter(num1: u8, num2: u8, num3: u8) {
    let numbers: [u8; 3] = [num1, num2, num3];
    for number in numbers.iter() {
        if number % 15 == 0 {
            println!("fizzbuzz");
        } else if number % 3 == 0 {
            println!("fizz");
        } else if number % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", number);
        };
    };
}

fn for_into_iter(num1: u8, num2: u8, num3: u8) {
    let numbers: [u8; 3] = [num1, num2, num3];
    println!("Your numbers are: {:?}", numbers);
    for number in numbers.into_iter() {
        if number % 15 == 0 {
            println!("fizzbuzz");
        } else if number % 3 == 0 {
            println!("fizz");
        } else if number % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", number);
        };
    };
    println!("Your numbers are: {:?}", numbers);
}

fn for_into_iter_mut(num1: u8, num2: u8, num3: u8) {
    let mut numbers = vec![num1, num2, num3];
    println!("Your numbers are: {:?}", numbers);
    for number in numbers.iter_mut() {
        if *number % 15 == 0 {
            *number = 15;
            println!("fizzbuzz");
        } else if *number % 3 == 0 {
            *number = 3;
            println!("fizz");
        } else if *number % 5 == 0 {
            *number = 5;
            println!("buzz");
        } else {
            println!("{}", number);
        };
    };
    println!("Your numbers are: {:?}", numbers);
}

fn main() {
    for_iter(1, 15, 33);
    for_into_iter(1, 15, 33);
    for_into_iter_mut(1, 30, 66);
}
