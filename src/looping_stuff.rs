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
