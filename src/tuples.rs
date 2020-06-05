pub fn run() {
    let person: (&str, &str, i8) = ("Gopi", "Mars", 4);

    println!("{} is from {} and is {} years old", person.0, person.1, person.2);
}