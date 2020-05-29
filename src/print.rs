pub fn run() {
    // Print to console
    println!("Hello from the print.rs");

    // Formatting
    println!("Number: {}", 1);
    println!("10 + 10 = {}", 10 + 10);
    // 1:1
    println!("{} is from {}", "Gopi", "Mars");
    // Positional
    println!(
        "{0} expands to {0} {1} {2}",
        "PHP", "Hypertext", "Preprocessor"
    );
    // Named
    println!(
        "{name} is {age} years old",
        name = "Baby",
        age = 2
    );

    // Traits
    println!("Decimal: {0} Binary: {0:b} Octal: {0:o} Hex: {0:x}", 10);
    // Debug trait
    println!("{:?}", (12, true, "hello"));
}