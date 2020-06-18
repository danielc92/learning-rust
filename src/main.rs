mod algorithm_stuff;
mod api_stuff;
mod array_stuff;
mod bool_stuff;
mod datetime_stuff;
mod error_handling;
mod function_stuff;
mod looping_stuff;
mod some_stuff;
mod struct_stuff;
mod token_stuff;
mod variable_stuff;

use std::io;

fn get_input(q: &str) -> io::Result<String> {
    println!("{}", q);

    let mut buffer: String = String::new();
    match io::stdin().read_line(&mut buffer) {
        Ok(x) => Ok(buffer.trim().to_string()),
        Err(error) => Err(error),
    }
}
fn main() {
    let age = get_input("How old are you?: ")
        .expect("DIDNT WORK")
        .parse::<i32>()
        .expect("FAILED TO PARSE");
    println!("Your age is {}", age);
}
