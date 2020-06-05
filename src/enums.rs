enum Movement {
    Up,
    Down,
    Left,
    Right
}

fn move_avatar(m: Movement) {
    match m { // similar to switch
        Movement::Up => println!("Avatar moving Up!"),
        Movement::Down => println!("Avatar moving Down!"),
        Movement::Left => println!("Avatar moving Left!"),
        Movement::Right => println!("Avatar moving Right!"),
    }
}

pub fn run() {
    move_avatar(Movement::Up);
    move_avatar(Movement::Down);
    move_avatar(Movement::Left);
    move_avatar(Movement::Right);
}