struct Person {
    name: String,
    age: u8,
}

// Implementation block
impl Person {
    // Method that takes a reference to self
    //self is the instance of the struct
    // We use &self to indicate that we are borrowing the instance
    fn greet(&self) { // Borrowing self because we don't need to modify the struct
        println!("Hello, my name is {}!", self.name);
    }

    // Method that modifies the struct, needs &mut self
    fn have_birthday(&mut self) {
        self.age += 1;
        println!("Happy Birthday! I am now {} years old.", self.age);
    }

    // Associated function (like static method)
    fn new(name: &str, age: u8) -> Person {
        Person {
            name: name.to_string(),
            age,
        }
    }
}

fn main() {
    // Using the associated function to create a new instance
    let mut alice = Person::new("Alice", 25); // mutable instance
    //Person instance created new with name Alice and age 25
    // Calling methods
    alice.greet();         // Borrowing self
    alice.have_birthday(); // Mutable borrow
}
