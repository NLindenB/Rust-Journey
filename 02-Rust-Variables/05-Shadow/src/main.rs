fn main() {
    let shadow_binding = 1;
  
    {
        println!("Before `Shadowing`: {}", shadow_binding);
  
        let shadow_binding = "one";
  
        println!("Shadowing applied in inner scope: {}", shadow_binding);
    };
    println!("Outer scope with value from line 2: {}", shadow_binding);
  
    let shadow_binding = 2;
  
    println!("Shadowing applied in outer scope: {}", shadow_binding);
}
    