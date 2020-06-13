mod array_stuff;
mod bool_stuff;
mod function_stuff;
mod struct_stuff;
mod variable_stuff;

fn main() {
    println!(
        "{} {} {} {} {}",
        function_stuff::area_of_square(2),
        function_stuff::area_of_square(3),
        function_stuff::area_of_square(4),
        function_stuff::area_of_square(0),
        function_stuff::area_of_square(-3)
    );

    let nums = [3, 54, 3, 52, 34, 423, 4];
    function_stuff::sum_array_nums(&nums);
}
