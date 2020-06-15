pub fn try_out_loops() {
    let mut c = 0;

    loop {
        c += 1;
        println!("{}", c);
        if c == 100 {
            break;
        }
    }
}

pub fn try_out_while() {
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
