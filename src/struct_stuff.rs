pub fn a_task() {
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
}

pub fn a_coord() {
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

pub fn a_basket() {
    #[derive(std::fmt::Debug)]
    struct Basket {
        some_ints: Vec<i32>,
        a_float_2: f64,
        a_name: String,
    }

    let x = Basket {
        some_ints: vec![32, 423, 4, 34],
        a_float_2: 3.4,
        a_name: "Daniel".to_owned(),
    };

    println!("{:?}", x);
}
