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

fn main() {
    println!(
        "{:?}",
        algorithm_stuff::binary_search(&[1, 2, 3, 4, 5, 6, 7, 8, 10, 13], 7)
    );

    println!(
        "{:?}",
        algorithm_stuff::linear_search(&[1, 2, 3, 4, 5, 6, 7, 8, 10, 13], &10)
    );
}
