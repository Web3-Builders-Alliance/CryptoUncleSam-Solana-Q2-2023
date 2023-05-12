pub fn run(){
    greeting("Hello", "Luis");
    // Bind function values to variables
    let get_sum = add(6, 9);
    println!("Sum: {}", get_sum);

    // Closure
    let n3: i32 = 10;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3; // Closure
    println!("C Sum: {}", add_nums(4, 2));
}

fn greeting(greet: &str, name: &str){
    println!("{} {}, nice to meet you!", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
    // No semicolon means it's an expression
    n1 + n2
}