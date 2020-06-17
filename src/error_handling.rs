pub fn string_to_i32(user_input: &String) -> String {
    let result = user_input.parse::<i32>();
    match result {
        Ok(i32) => "Success".to_owned(),
        Err(i32) => "Failure".to_owned(),
    }
}
