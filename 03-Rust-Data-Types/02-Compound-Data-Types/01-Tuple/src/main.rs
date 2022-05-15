fn main() {
	let tup = (5, 'x', true);
	let first = tup.0;
	let second = tup.1;
	let last = tup.2;
	println!("The first value in the tuple is {}", first);
	println!("The second value in the tuple is {}", second);
	println!("The last value in the tuple is {}", last);
    // Change the symbols inside the {} with `:?` for a normal print and `#?` for pretty print
    println!("The tuple is {:#?}", tup);
}
