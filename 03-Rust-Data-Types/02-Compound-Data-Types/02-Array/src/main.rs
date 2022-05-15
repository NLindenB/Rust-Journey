// The `Type Error` Example for printing an array without the coorect syntax
// fn main() {
// 	let arr = [5, 6, 7];
// 	let first = arr[0];
// 	let second = arr[1];
// 	let last = arr[2];
// 	println!("The first value in the array is {}", first);
// 	println!("The second value in the array is {}", second);
// 	println!("The last value in the array is {}", last);
//     println!("The array is {}", arr);
// }

// Example of array immutability / `fixed length set`
// fn fixed_size_arr() {
//     let mut arr = [5, 6, 7];
//     arr += 8;
//     print!("The array is {:?}", arr)
// }

// Example to print an array with the correct syntax
fn main() {

    //fixed_size_arr();
	let arr = [5, 6, 7];
	let first = arr[0];
	let second = arr[1];
	let last = arr[2];
	println!("The first value in the array is {}", first);
	println!("The second value in the array is {}", second);
	println!("The last value in the array is {}", last);
    // Change the symbols inside the {} with `:?` for a normal print and `#?` for pretty print
    println!("The array is {:?}", arr);
}
