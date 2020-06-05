fn fizz_buzz(x:i32) {
    if x % 15 == 0 {
        println!("{}\tFizzBuzz", x);
    } else if x % 3 == 0 {
        println!("{}\tFizz", x);
    } else if x % 5 == 0 {
        println!("{}\tBuzz", x);
    } else {
        println!("{}", x);
    }
}

pub fn run() {
    // general loop
    let mut count = 0;
    loop {
        count += 1;
        println!("Number: {}", count);

        if count == 20 {
            break;
        }
    }

    // while loop
    while count <=100 {
        fizz_buzz(count);

        count += 1;
    }

    // for range loop
    for x in 0..100 {
        fizz_buzz(x);
    }
}