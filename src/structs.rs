// Normal struct
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// Tuple struct
struct TColor(u8, u8, u8);

// Complex struct
struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }

    fn full_name(&self) -> String { // use &self to avoid reference moving to this function and fail further
        format!("{} {}", &self.first_name, &self.last_name)
    }
}

// custom print (toString override) by implementing `std::fmt::Display` trait
impl std::fmt::Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "My name is {} {}", &self.first_name, &self.last_name)
    }
}

// custom trait
trait Greeting {
    fn greet(&self, greeting: &str) -> String;
}

impl Greeting for Person {
    fn greet(&self, greeting: &str) -> String {
        format!("{}, {}!", greeting, &self.full_name())
    }
}

pub fn run() {
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0,
    };

    c.green = 255;

    println!("Color: {} {} {}", c.red, c.green, c.blue);

    let mut tc = TColor(255, 0, 0);

    tc.1 = 255;

    println!("Color: {} {} {}", tc.0, tc.1, tc.2);

    let p = Person::new("Gopi", "Sathya");
    println!("Person says, \"{}\"", p);
    println!("Person says, \"{}\"", p.full_name());
    println!("{}", p.greet("Hello there"));
}