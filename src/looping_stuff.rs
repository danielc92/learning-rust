pub fn loop_with_break() {
    let mut c = 0;

    loop {
        c += 1;
        println!("{}", c);
        if c == 100 {
            break;
        }
    }
}

pub fn loop_array_with_an_index() {
    let mut my_array: [i32; 10] = [1, 123, 32, 4, 33, 5235, 4325, 345, 23, 3];

    for (index, item) in my_array.iter().enumerate() {
        println!("{} {}", item, index);
    }
}

pub fn while_with_break() {
    let mut flag: bool = true;
    let mut c = 0;
    while flag {
        println!("{}", c);
        c += 2;
        if c == 150 {
            flag = false;
        }
    }
}
