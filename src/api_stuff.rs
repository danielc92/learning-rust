/*
Usage
#[tokio::main]
async fn main() -> Result<(), Error> {
    get_todos().await?;
    Ok(())
}
*/

use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct User {
    userId: i32,
    id: i32,
    title: String,
    completed: bool,
}

pub async fn get_todos() -> Result<Vec<User>, Error> {
    let api_url = "https://jsonplaceholder.typicode.com/todos";
    let response = reqwest::get(api_url).await?;
    let users: Vec<User> = response.json().await?;
    println!("{:?}", users);
    Ok(users)
}
