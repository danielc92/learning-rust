enum Direction {
    Up,
    Left,
    Right,
    Down,
}

pub fn display_dir() {
    let dir = Direction::Down;

    match dir {
        Direction::Up => println!("Going up!"),
        Direction::Down => println!("Going down!"),
        Direction::Right => println!("Going right1"),
        Direction::Left => println!("Going left!"),
    }
}
