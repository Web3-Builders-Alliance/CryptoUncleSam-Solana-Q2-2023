enum Movement {
    // Variants
    Up,
    Down,
    Left,
    Right
}

fn move_avatar(m: Movement, n: i32){
    // Perform action depending on movement
    match m {
        Movement::Up => println!("Avatar{} moving up ", n),
        Movement::Down => println!("Avatar{} moving down", n),
        Movement::Left => println!("Avatar{} moving left", n),
        Movement::Right => println!("Avatar{} moving right", n)
    }
}

pub fn run(){
    let avatar1 = Movement::Left;
    let avatar2 = Movement::Up;
    let avatar3 = Movement::Right;
    let avatar4 = Movement::Down;

    move_avatar(avatar1, 1);
    move_avatar(avatar2, 2);
    move_avatar(avatar3, 3);
    move_avatar(avatar4, 4);
}