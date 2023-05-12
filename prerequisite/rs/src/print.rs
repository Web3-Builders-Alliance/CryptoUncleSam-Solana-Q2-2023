pub fn run(){
    // Print to console
    println!("Hello from the print.rs file");

    // BAsic formatting
    println!("{} is from {}", "Luis", "Nicaragua");

    // Positional arguments
    println!("{0} is from {1} and {0} likes to {2}", "Luis", "Nicaragua", "code");

    // Named arguments
    println!("{name} likes to play {activity}", name = "Luis", activity = "Baseball");

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 3, 3, 3);

    // Placeholder for debug trait
    println!("{:?}", (5, false, "bye"));

    // Basic math
    println!("4 + 2 = {}", 4 + 2);
}