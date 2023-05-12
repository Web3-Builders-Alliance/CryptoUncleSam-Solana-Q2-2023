// Traditional Struct
struct Color {
    red: u8,
    green: u8,
    blue: u8
}

struct Person {
    first_name: String,
    last_name: String,
}

// Implement a struct into fn
impl Person {
    // Construct person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }

    // Get full name
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    // Set last name
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    // Name to tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }

    // Get first name
    fn get_first_name(&self) -> &str {
        &self.first_name
    }

    // Get last name
    fn get_last_name(&self) -> &str {
        &self.last_name
    }
}

pub fn run(){
    // Create a color
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0
    };

    // Change the color
    c.red = 40;
    println!("Color: {} {} {}", c.red, c.green, c.blue);

    // Create a new person
    let mut p = Person::new("Luis", "Perez");
    println!("Person: {} {}", p.first_name, p.last_name);
    
    // Set last name
    p.set_last_name("Galindo");
    println!("Person: {} {}", p.first_name, p.last_name);

    // Print full name
    println!("Person: {}", p.full_name());

    // Build a new person
    let p2 = Person::new("Ana", "Sans");
    println!("Person: {}", p2.full_name());

    // Get first name
    println!("Person: {}", p2.get_first_name());

    // Get last name
    println!("Person: {}", p2.get_last_name());

    // Get full name
    println!("Person: {}", p2.full_name());

    // Convert person to tuple
    println!("Person Tuple: {:?}", p2.to_tuple());
}