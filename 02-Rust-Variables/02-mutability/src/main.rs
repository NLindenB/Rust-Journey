// Example of error for mutability
// fn main() {
//     let x = 5;
//     x += 1;
//     println!("x is now {}", x);
//   }

fn main() {
    let mut x = 5;
    x += 1;
    println!("x is now {}", x);
}
