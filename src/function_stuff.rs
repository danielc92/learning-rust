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

pub fn loop_until(number: i32) {
    let mut counter: i32 = 0;
    loop {
        counter += 1;
        println!("Current counter is {}", counter);
        if counter == number {
            println!("Broke the loop.");
            break;
        }
    }
}

pub fn match_student(grade: u16) {
    match grade {
        1 | 2 | 3 | 4 | 5 | 6 => println!("Primary school."),
        7 | 8 | 9 | 10 => println!("Secondary school."),
        11 | 12 => println!("Secondary school (VCE)."),
        _ => println!("No case met."),
    }
}

pub fn if_elsing(name: &String) -> String {
    if name.len() <= 4 {
        return "short name length".to_owned();
    } else if name.len() <= 8 {
        return "regular name length".to_owned();
    } else {
        return "really long name".to_owned();
    };
}
