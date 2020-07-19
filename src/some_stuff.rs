pub fn validate_name(name: &String) -> Option<String> {
    if name.len() > 10 {
        Some("Successful validation!".to_owned())
    } else {
        None
    }
}

pub fn validate_number(number: &i32) -> Option<String> {
    let lower: i32 = 10;
    let upper: i32 = 50;
    if number > &lower && number < &upper {
        Some("Successful validation".to_string())
    } else {
        None
    }
}

pub fn add_to_option(x: Option<i32>) -> Option<i32> {
    match x {
        Some(x) => Some(x + 10),
        None => None,
    }
}

// Usage
// let result = some_stuff::validate_number(&32);
// match result {
//     None => println!("Failed"),
//     Some(x) => println!("{}", x),
// }

fn return_enrolled(name: &str) -> Option<&str> {
    match name {
        "Daniel" => Some("Daniel is enrolled"),
        "Jeffrey" => Some("Coolio"),
        _ => None,
    }
}
