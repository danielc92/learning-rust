pub fn string_to_i32(user_input: &String) -> String {
    let result = user_input.parse::<i32>();
    match result {
        Ok(i32) => "Success".to_owned(),
        Err(i32) => "Failure".to_owned(),
    }
}

pub fn file_read_to_succeed() -> String {
    match std::fs::File::open("C:\\Users\\dc\\Desktop\\png\\add.png") {
        Ok(file) => "Found the file".to_string(),
        Err(err) => "Error occured".to_string(),
    }
}

pub fn file_read_to_fail() -> String {
    match std::fs::File::open("8649849849") {
        Ok(file) => "Found the file".to_string(),
        Err(err) => "Error occured".to_string(),
    }
}
