fn main() {
    //Integer variable declaration
    let x: i32 = 10; //i32 is a 32-bit signed integer
    println!("The value of x is: {}", x);
    //Unsigned integer variable declaration
    let u: u32 = 15; //u32 is a 32-bit unsigned integer
    println!("The value of u is: {}", u);
    //Floating point variable declaration
    let y: f64 = 20.5; //f64 is a 64-bit floating point
    println!("The value of y is: {}", y);
    //Boolean variable declaration
    let is_active: bool = true;
    println!("Is active: {}", is_active);
    //Character variable declaration
    let letter: char = 'A';
    println!("The letter is: {}", letter);
    //String variable declaration
    //String is stored in heap memory which is dynamically allocated(dynamic allocated memory means memory is allocated at runtime)
    let name: &str = "Hello, Rust!"; //&str is a string slice(immutable and it points to a sequence of UTF-8 characters)
    println!("The name is: {}", name);
    //Array variable declaration
    //           [type; size]
    let numbers: [i32; 5] = [1, 2, 3, 4, 5]; //Array of 5 integers
    println!("The numbers are: {:?}", numbers); //{:?} is used to print the array
    //Tuple variable declaration
    let person: (&str, i32) = ("Alice", 30); //Tuple with a string slice and an integer
    println!("Name: {}, Age: {}", person.0, person.1);
    //Struct variable declaration
    struct Point {
        x: f64,
        y: f64,
    }
    let point = Point { x: 10.0, y: 20.0 };
    println!("Point coordinates: ({}, {})", point.x, point.y);
    //list variable declaration using vector
    let mut vec_numbers: Vec<i32> = Vec::new(); //Creating an empty vector
    vec_numbers.push(1); //Adding elements to the vector
    vec_numbers.push(2);
    vec_numbers.push(3);
    println!("Vector numbers: {:?}", vec_numbers);
}
