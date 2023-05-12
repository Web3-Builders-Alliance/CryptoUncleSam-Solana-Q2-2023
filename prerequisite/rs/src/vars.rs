pub fn run() {
    let name = "Luis";
    let mut age = 30;
    println!("My name is {} and I am {}", name, age);
    age = 38;
    println!("My name is {} and I am {}", name, age);

    // Define constant
    const ID: i32 = 007;
    println!("ID: {}", ID);

    // Assign multiple vars
    let (my_name, my_age) = ("Luis", 30);
    println!("{} is {}", my_name, my_age);
}