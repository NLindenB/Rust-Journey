fn arr_slice() {
    let arr: [i8; 4] = [5, 6, 7, 8];
    let slice_of_arr = &arr[0..2];
    println!("The slice of the array is {:?}", slice_of_arr);
}


fn main() {

    arr_slice();
    let string: &str = "Hello, World!";
    println!("My first {} in Rust", string);
}
