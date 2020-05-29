// variables hold primitive data or reference to data
// variables are immutable by default
// Rus is a block-scoped language

pub fn run() {
    let name = "Gopi"; // immutable
    let mut age = 4;    // mutable

    println!("My name is {} and {} years old", name, age);
    age = 5;
    println!("My name is {} and {} years old", name, age);

    // Constants require types
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple variables
    let (m_name, m_age) = ("Baby", 2);

    println!("My name is {} and {} years old", m_name, m_age);
}