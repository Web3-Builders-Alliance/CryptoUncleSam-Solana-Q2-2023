pub fn run() {
    let mut bye = String::from("Bye ");

    // Get length
    println!("Length: {}", bye.len());

    // Push char
    bye.push('W');

    // Push string
    bye.push_str("orld!");

    // Capacity in bytes
    println!("Capacity: {}", bye.capacity());

    // Check if empty
    println!("Is Empty: {}", bye.is_empty());

    // Contains
    println!("Contains 'World'? {}", bye.contains("World"));

    // Replace
    println!("Replace: {}", bye.replace("World", "There"));

    // Loop through string by whitespace
    for word in bye.split_whitespace() {
        println!("{}", word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    // Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{}", s);
}