// Loops - Used to iterate until a condition is met

pub fn run() {
    let mut count: i8 = 0;

    // infinite loop
    loop {
        count += 1;
        println!("Number: {}", count);

        if count == 20 {
            break;
        }
    }

    // While loop (FizzBuzz)
    println!("While Loop");
    while count <= 100 {
        if count % 15 == 0 {
            println!("fizzbuzz");
        } else if count % 3 == 0 {
            println!("fizz");
        } else if count % 5 == 0 {
            println!("buzz");
        } else {
            println!("");
        }

        // Increment
        count += 1;
    }

    // for loop
    println!("For loop");
    for x in 0..100 {
        if x % 15 == 0 {
            println!("fizzbuzz");
        } else if x % 3 == 0 {
            println!("fizz");
        } else if x % 5 == 0 {
            println!("buzz");
        } else {
            println!("");
        }
    }
}
