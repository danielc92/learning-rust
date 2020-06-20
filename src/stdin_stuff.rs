use std::io;

pub fn get_input(q: &str) -> io::Result<String> {
    println!("{}", q);

    let mut buffer: String = String::new();
    match io::stdin().read_line(&mut buffer) {
        Ok(x) => Ok(buffer.trim().to_string()),
        Err(error) => Err(error),
    }
}

//usage

// let age = get_input("How old are you?: ")
// .expect("DIDNT WORK")
// .parse::<i32>()
// .expect("FAILED TO PARSE");
// println!("Your age is {}", age);
