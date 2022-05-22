fn main() {
    let long_lived_binding = 1;

    {
        let short_lived_binding = 2;

        println!("Smaller scope with short-lived var: {}", short_lived_binding);
    }

    // Uncomment the below line of code to observe the scope-error
    //println!("Main scope with short-lived var: {}", short_lived_binding);
    println!("Main scope with long-lived var: {}", long_lived_binding);

}
