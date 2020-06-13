mod array_stuff;
mod bool_stuff;
mod function_stuff;
mod struct_stuff;
mod variable_stuff;

fn main() {
    let name = "Daniel Corcoran".to_owned();
    let result = function_stuff::if_elsing(&name);
    print!("{}", result);
}
