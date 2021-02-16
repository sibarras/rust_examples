pub fn run() {
    // Print to console
    println!("Hello world!");

    // You have to use a placeholder to print nums
    println!("{} is the age of {}", 25, "Sam");

    // Positional Arguments
    println!("{0} is from {1} and {0} likes to {2}", "Brad", "Mass", "code");

    // Named Arguments
    println!("{name} likes to play {activity}",
        name = "John",
        activity = "Baseball"
    );

    // Placeholder Traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    // Basic Math
    println!("10 + 10 = {}", 10+10);
}