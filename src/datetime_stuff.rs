use std::time::SystemTime;

pub fn run() {
    // let dur: Duration = Duration::new(10, 0);
    let new_time = SystemTime::now();
    println!("{:?}", new_time);
}
