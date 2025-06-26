enum Direction {
    North,
    East,
    South,
    West,
}

fn main() {
    let my_direction = Direction::North;
    check(my_direction);
    
}

// Pattern matching
fn check(direction:Direction) {
    match direction {
        Direction::North=>print!("North direction"),
        Direction::East=>print!("East Direction"),
        _default=>print!("None in the given")
    }
}

