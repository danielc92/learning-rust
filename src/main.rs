// fn main() {
//     let is_awake: bool = false;
//     let is_eating: bool = true;
//     let is_watching: bool = false;

//     println!("Result: {}", is_awake);
//     println!("Result: {}", is_awake || is_eating);
//     println!("Result: {}", is_eating && is_awake);
//     println!("Result: {}", is_eating && !is_awake);
// }

// Variables
// fn main() {
//     // signed types
//     let signed_num_1: i8 = 20;
//     let signed_num_2: i16 = -32500;
//     let signed_num_3: i32 = 32500;
//     let signed_num_4: i64 = 9223372036854775807;
//     let signed_num_5: i128 = -170141183460469231731687303715884105728;
//     // unsigned types
//     let unsigned_num_0: u8 = 0;
//     let unsigned_num_1: u8 = 255;
//     let unsigned_num_2: u16 = 65535;
//     let unsigned_num_3: u32 = 4294967295;
//     let unsigned_num_4: u64 = 18_446_744_073_709_551_615;
//     let unsigned_num_5: u128 = 23;
// }

// Arrays and tuples
// fn main() {
//     let num_array = [2, 3, 4];

//     let num_tuple = (3, 5, 4);

//     // Fixed size array declaration
//     let fixed_array: [i32; 3] = [1, 2, 1];

//     // Return array length
//     println!("{}", num_array.len());

//     // Return value at index
//     println!("{}", num_array[1]);

//     // Check if number is in array
//     let num = 3;
//     let result = num_array.contains(&num);
//     println!("{}", result);
// }

// Custom types
fn main() {
    struct Task {
        is_complete: bool,
        title: String,
        timestamp: u64,
    }

    let mut daniels_task: Task = Task {
        is_complete: false,
        title: "First task of the day".to_string(),
        timestamp: 156465465456,
    };

    println!(
        "Daniels task {} {} {}",
        daniels_task.is_complete, daniels_task.title, daniels_task.timestamp
    );

    daniels_task.title = "I changed my mind".to_string();

    println!(
        "Daniels task {} {} {}",
        daniels_task.is_complete, daniels_task.title, daniels_task.timestamp
    );

    struct Coordinate {
        longitude: f64,
        latitude: f64,
    }

    let hometown: Coordinate = Coordinate {
        longitude: 144.5456,
        latitude: -37.654,
    };

    println!(
        "Hometown coordinates {} {}",
        hometown.longitude, hometown.latitude
    );
}
