pub fn area_of_square(length: i32) -> i32 {
    if length <= 0 {
        return -1;
    }
    return length * length;
}

pub fn sum_array_nums(arr: &[i32]) {
    println!("Length of array is {}", arr.len());

    let mut sum: i32 = 0;

    for elem in arr {
        sum += elem;
        println!("{}", elem);
    }

    println!("Sum is {}", sum);
}

// println!(
//     "{} {} {} {} {}",
//     function_stuff::area_of_square(2),
//     function_stuff::area_of_square(3),
//     function_stuff::area_of_square(4),
//     function_stuff::area_of_square(0),
//     function_stuff::area_of_square(-3)
// )
