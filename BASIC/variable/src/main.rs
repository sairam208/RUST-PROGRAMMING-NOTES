fn main() {
    let x: i32 = 10;//i32 is a data type that represents a 32-bit signed integer
    let y = 20;//type inference: Rust can automatically infer the type of y as i32      
    let sum: i32 = x + y;
    println!("The sum of {} and {} is {}", x, y, sum); //{} is a placeholder for variable values
    let message="Hello, Rust!";//&str is a string slice type
    println!("{}", message);
    //slicing a string
    let first_five = &message[0..5];//slicing the first 5 characters
    println!("Sliced string: {}", first_five);
    //slicing  by skipping characters
    let skip_chars= &message[0..message.len()].chars().step_by(2).collect::<String>();
    println!("String with skipped characters: {}", skip_chars);
    //mutable variable
    let mut count = 0;//mut keyword makes the variable mutable
    count += 1;
    println!("Count after incrementing: {}", count);
    let _y= 15;//underscore prefix indicates that the variable is intentionally unused
    //if a variable is unused without an underscore, the compiler will issue a warning not an error
    //constant variable
    const MAX_POINTS: u32 = 100_000;//const keyword defines a constant variable
    println!("The maximum points are: {}", MAX_POINTS);
    //array variable    
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];//array of 5 integers
    println!("The first number in the array is: {}", numbers[0]);
    //assertion
    assert!(sum == 30, "Sum should be 30");//assert! macro checks if the condition is true 
    //if the condition is false, it panics with the provided message
}
//println! is a macro that prints formatted text to the console