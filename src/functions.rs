// Functions - Used to store blocks of code for re-use

pub fn run() {
    greetins("hello", "Jane");
    
    // Bind function values to variables
    let get_sum_1:i32 = add(5, 5);
    let get_sum_2:i32 = add2(5, 5);
    let get_sum_3:i32 = add3(5, 5);

    println!("Sums: 1->{}; 2->{}; 3->{};", get_sum_1, get_sum_2, get_sum_3);

    // Closures
    let n3:i32 = 10;
    let add_nums = |n1:i32, n2:i32| n1 + n2 + n3;
    println!("Closure Sum: {}", add_nums(3, 3));
}

fn greetins(greet: &str, name:&str) {
    println!("{} {}, nice to meet you!", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
    let result:i32 = n1 + n2;
    return result
}

fn add2(n1: i32, n2: i32) -> i32 {
    return n1 + n2;
}

fn add3(n1: i32, n2: i32) -> i32 {
    n1 + n2
}