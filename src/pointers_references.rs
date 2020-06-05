pub fn run() {
    // primitive array
    let arr1 = [1, 2, 3];
    let arr2 = arr1;

    println!("Values: {:?}", (arr1, arr2));

    // With non primitives, if a piece of data is assigned to another variable, the first variable
    // will no longer hold that value. WIll have to use a reference (&) to point to the resource

    // vectors
    let vec1 = vec![1, 2, 3];

    // let vec2 = vec1;                        // wrong waY
    // println!("Values: {:?}", (vec1, vec2)); // wrong way
    /*
    let vec1 = vec![1, 2, 3];
        ---- move occurs because `vec1` has type `std::vec::Vec<i32>`, which does not implement the `Copy` trait
    let vec2 = vec1;
            ---- value moved here

    println!("Values: {:?}", (vec1, vec2));
                              ^^^^ value used here after move
    */
    let vec2 = &vec1;              // correct way
    println!("Values: {:?}", (&vec1, vec2)); // correct way
}