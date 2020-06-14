pub fn try_it_out() {
    let num_array = [2, 3, 4];
    let num_tuple = (3, 5, 4);
    // Fixed size array declaration
    let fixed_array: [i32; 3] = [1, 2, 1];
    // Return array length
    println!("{}", num_array.len());
    let mut mutable_array: [i8; 4] = [1, 2, 2, 3];
    mutable_array[3] = 1;
    println!("{:?}", mutable_array);

    // Return value at index
    println!("{}", num_array[1]);
    // Check if number is in array
    let num = 3;
    let result = num_array.contains(&num);
    println!("{}", result);

    // Vectors can vary in length, unlike arrays
    let mut my_vec: Vec<i64> = vec![3, 32, 34234, 234, 234, 234, -2];
    my_vec.push(3);
    println!("{:?}", my_vec);

    #[derive(Debug)]
    struct DeskItem {
        name: String,
        weight_grams: u16,
        height_cm: u16,
        width_cm: u16,
    }

    let pencil: DeskItem = DeskItem {
        name: "Pencil".to_owned(),
        weight_grams: 30,
        height_cm: 32,
        width_cm: 123,
    };

    let lamp: DeskItem = DeskItem {
        name: "Lamp".to_owned(),
        weight_grams: 12,
        height_cm: 22,
        width_cm: 32,
    };

    println!("{:?}", lamp);

    let desk_items: Vec<DeskItem> = vec![pencil, lamp];
}
