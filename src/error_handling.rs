pub fn file_read_to_succeed() -> String {
    match std::fs::File::open("C:\\Users\\dc\\Desktop\\png\\add.png") {
        Ok(_file) => "Found the file".to_string(),
        Err(_err) => "Error occured".to_string(),
    }
}

pub fn file_read_to_fail() -> String {
    match std::fs::File::open("8649849849") {
        Ok(_file) => "Found the file".to_string(),
        Err(_err) => "Error occured".to_string(),
    }
}
