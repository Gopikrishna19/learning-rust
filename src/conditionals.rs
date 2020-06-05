pub fn run() {
    let age = 18;

    // If else
    if age >= 21 {
        println!("Bar: You can enter");
    } else {
        println!("Bar: You are not old enough");
    }

    // shorthand if else
    let is_old_enough = if age >= 18 { true } else { false };
    println!("Is old enough: {}", is_old_enough);
}