// Structs - Used to create custom data types

// Traditional struct
struct Color1 {
    red: u8,
    green: u8,
    blue: u8,
}

// Tuple struct
struct Color2(u8, u8, u8);

// Person Struct
struct Person {
    first_name: String,
    last_name: String
}

impl Person {
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }

    // get full name
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    // Set last name
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    //  Name to tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run1() {
    let mut c = Color1 {
        red: 255,
        green: 0,
        blue: 0,
    };
    c.red = 200;
    println!("Color: {} {} {}", c.red, c.green, c.blue);
}

pub fn run2() {
    let mut c = Color2(255, 0, 0);
    c.0 = 200;
    println!("Color: {} {} {}", c.0, c.1, c.2);
}

pub fn run() {
    run1();
    run2();
    
    let mut p = Person::new("John", "Doe");
    println!("Person: {}", p.full_name());

    p.set_last_name("Williams");
    println!("Person: {}", p.full_name());

    println!("Person Tuple: {:?}", p.to_tuple());

}