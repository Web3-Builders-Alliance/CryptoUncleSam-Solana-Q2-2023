use std::env;

pub fn run(){
    let args: Vec<String> = env::args().collect();

    println!("Args: {:?}", args);

    let command = args[1].clone();
    let name = "Luis";
    let status = "69%";
    println!("Command: {}", command);

    if command == "bye" {
        println!("Bye {}, see you soon", name);
    } else if command == "status" {
        println!("Status is {}", status);
    } else {
        println!("That is not a valid command");
    }
}