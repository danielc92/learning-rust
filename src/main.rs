mod algorithm_stuff;
mod array_stuff;
mod bool_stuff;
mod datetime_stuff;
mod function_stuff;
mod looping_stuff;
mod some_stuff;
mod struct_stuff;
mod token_stuff;
mod variable_stuff;

use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct User {
    userId: i32,
    id: i32,
    title: String,
    completed: bool,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let response = reqwest::get("https://jsonplaceholder.typicode.com/todo5s").await?;
    println!("{}", response.status());
    let users: Vec<User> = response.json().await?;
    println!("{:?}", users);
    Ok(())
}

// https://jsonplaceholder.typicode.com/todos
