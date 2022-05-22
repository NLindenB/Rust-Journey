use std::io;

fn main() {
  loop {
    println!("What is the coolest programming language?");
    let mut answer = String::new();
    
    io::stdin().read_line(&mut answer).expect("Failed to read line");
    
    if answer.trim() == "rust" {
      break println!("You got it! Rust is the answer to all question in universe, just like 42!");
    };
  };
}
