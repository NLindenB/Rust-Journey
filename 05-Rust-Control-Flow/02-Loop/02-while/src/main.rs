use std::io;

fn main() {
  let mut answer = String::new();
  while answer.trim() != "rust" {
    println!("What is the coolest programming language?");
    answer = String::new();
    io::stdin().read_line(&mut answer).expect("Failed to read line");
  };
  
  println!("You got it! Rust is the answer to all question in universe, just like 42!");
}
