fn main() {
    // Mutable references must be used in separate scopes
    // to avoid simultaneous mutable borrows.
    // This code demonstrates correct usage of mutable references
    //Println! macro to display values
    println!("Hello Rust!");
    //USe cargo to run this code
    //Create a new Rust project with `cargo new project_name`
    let (mut x, mut y) = (5, 10);
    {
        let r1 = &mut x;
        *r1 += 1;
    }
    {
        let r2 = &mut y;
        *r2 += 1;
    }
    println!("x: {}, y: {}", x, y);

}
