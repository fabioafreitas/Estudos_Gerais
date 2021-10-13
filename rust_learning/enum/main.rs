enum Direction {
    Up, Down, Left, Right
}

fn main() {
    let test: Direction = Direction::Down;

    match test {
        Direction::Up => println!("Up"),
        Direction::Down => println!("Down"),
        Direction::Left => println!("Left"),
        Direction::Right => println!("Right"),
    }
}