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
