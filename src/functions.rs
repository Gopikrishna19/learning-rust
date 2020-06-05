pub fn run() {
    greeting("Hello", "Gopi");

    let sum = add(5, 5);
    println!("Sum: {}", sum);

    // closures
    let n3 = 10;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("Closure sum: {}", add_nums(3, 2));
}

fn greeting(greet: &str, name: &str) {
    println!("{}, {}!", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}