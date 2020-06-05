// Arrays are fixed list with same type elements

pub fn run() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    println!("{:?}", numbers);

    // single value
    numbers[2] = 6;
    println!("At pos 2: {}", numbers[2]);

    println!("Length: {}", numbers.len());
    println!("Array uses: {}", std::mem::size_of_val(&numbers));

    // slicing
    let slice = &numbers[0..2];
    println!("Slice: {:?}", slice);
}