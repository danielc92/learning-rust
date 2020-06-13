pub fn try_it_out() {
    let num_array = [2, 3, 4];
    let num_tuple = (3, 5, 4);
    // Fixed size array declaration
    let fixed_array: [i32; 3] = [1, 2, 1];
    // Return array length
    println!("{}", num_array.len());
    // Return value at index
    println!("{}", num_array[1]);
    // Check if number is in array
    let num = 3;
    let result = num_array.contains(&num);
    println!("{}", result);
}
