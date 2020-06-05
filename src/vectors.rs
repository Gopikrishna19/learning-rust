// Vectors are resizable arrays

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    println!("{:?}", numbers);

    // single value
    numbers[2] = 6;
    println!("At pos 2: {}", numbers[2]);

    println!("Length: {}", numbers.len());
    println!("Vector uses: {}", std::mem::size_of_val(&numbers));

    // slicing
    let slice = &numbers[0..2];
    println!("Slice: {:?}", slice);

    // resizing
    numbers.push(6);
    numbers.push(7);
    numbers.pop();

    println!("{:?}", numbers);

    // loop
    for x in numbers.iter() {
        println!("Number in vec: {}", x);
    }

    // loop and mutate
    for x in numbers.iter_mut() {
        *x += 2;
    }
    println!("After mutation: {:?}", numbers);
}