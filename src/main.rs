fn main() {
    let is_awake: bool = false;
    let is_eating: bool = true;
    let is_watching: bool = false;

    println!("Result: {}", is_awake);
    println!("Result: {}", is_awake || is_eating);
    println!("Result: {}", is_eating && is_awake);
    println!("Result: {}", is_eating && !is_awake);
}
