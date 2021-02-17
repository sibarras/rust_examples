pub fn run() {
    // General Variable (Canot be mutated)
    let name = "Brad";
    // Mutable variable
    let mut age = 37;
    println!("My name is {} and I am {}", name, age);
    age = 38;
    println!("My name is {} and I am {}", name, age);

    // Define Constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assingn Multiple Variables
    let ( my_name, my_age ) = ("Brad", 37);
    println!("{} is {}", my_name, my_age);
}