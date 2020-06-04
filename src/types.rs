/**
Primitive types:
    Integers: (u|i)(8,16,32,64,128)
    Floats: f32, f64
    Boolean: bool
    Characters: char
    Tuples
    Arrays
 */
/**
Rust is statically typed
Must know all types at compile time
Can infer the types based on the value

Ex:
    let x = 1;      // i32;
    let y = 2.5;    // f64;
*/

pub fn run() {
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // implicit
    let x = 1;
    // explicit
    let y: bool = true;
    // char
    let c = 'a';
    // unicode
    let u = '\u{1F601}';
    // string
    let s = "string";

    println!("{:?}", (x, y, c, u, s));
}