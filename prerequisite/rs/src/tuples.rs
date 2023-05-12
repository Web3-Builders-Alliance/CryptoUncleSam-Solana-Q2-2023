pub fn run(){
    // Tuples group together values of different types
    // Max 12 elements

    let person: (&str, &str, i8) = ("Luis", "Nicaragua", 30);

    println!("{} is from {} and is {}", person.0, person.1, person.2);
}