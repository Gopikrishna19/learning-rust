// Two types:
//  Primitive string is immutable fixed length
//  Growing string, heap allocated data structure

pub fn run() {
    let primitive = "primitive";

    println!("{}", primitive);
    println!("Length: {}", primitive.len());

    let mut data_structure = String::from("Hello ");

    println!("{}", data_structure);

    // push character
    data_structure.push('W');
    println!("{}", data_structure);
    println!("Length: {}", data_structure.len());
    println!("Capacity: {}", data_structure.capacity());

    // push string
    data_structure.push_str("ar!");
    println!("{}", data_structure);

    println!("Length: {}", data_structure.len());
    println!("Capacity: {}", data_structure.capacity());
    println!("IsEmpty: {}", data_structure.is_empty());
    println!("Replace: {}", data_structure.replace("War", "World"));
    println!("Contains 'War': {}", data_structure.contains("War"));

    // split
    for word in data_structure.split_whitespace() {
        println!("{}", word);
    }

    // Create string with capacity
    // Capacity increases by doubling as the string grows
    // with_capacity creates a string with preAllocated buffer
    let mut string_with_capacity = String::with_capacity(10);

    string_with_capacity.push('a');
    string_with_capacity.push('b');

    println!("{}", string_with_capacity);
}