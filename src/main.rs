mod array_stuff;
mod bool_stuff;
mod datetime_stuff;
mod function_stuff;
mod looping_stuff;
mod some_stuff;
mod struct_stuff;
mod token_stuff;
mod variable_stuff;

fn main() {
    let result = some_stuff::validate_number(&32);
    match result {
        None => println!("Failed"),
        Some(x) => println!("{}", x),
    }

    let result = some_stuff::validate_number(&555);
    match result {
        None => println!("Failed"),
        Some(x) => println!("{}", x),
    }

    let result = some_stuff::validate_number(&-15);
    match result {
        None => println!("Failed"),
        Some(x) => println!("{}", x),
    }
}
