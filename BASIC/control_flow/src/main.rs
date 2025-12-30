fn main() {
    let check1:bool=false;
    let check2:bool=true;
    //if else  else if statement
    if check1{
        println!("The condition check1 is true");
    }
    else if check2{
        println!("The condition check1 is false and check2 is true");
    }
    else{
        println!("The condition check1 and check2 are false");
    }
    //Using match statement
    let number=i32::from(check2); //Converting bool to integer using from function
    match number {
        0 => println!("The condition check2 is false"),
        1 => println!("The condition check2 is true"),
        _ => println!("Invalid condition"), //_ is a wildcard pattern that matches any value not explicitly listed above
    }    
}
