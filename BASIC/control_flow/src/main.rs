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
    //from function converts true to 1 and false to 0
    match number {
        0 => println!("The condition check2 is false"),
        1 => println!("The condition check2 is true"),
        _ => println!("Invalid condition"), //_ is a wildcard pattern that matches any value not explicitly listed above
    }    
    //Using if in let statement
    let condition=true;
    let value=if condition {100} else {200}; //if condition is true then value is 100 else value is 200
    println!("The value is: {}",value);
    //While let statement
    let mut optional_value:Option<i32>=Some(0); //Option is an enum that can either be Some(value) or None
    while let Some(i)=optional_value{
        if i>5{
            optional_value=None; //Setting optional_value to None to break the loop
        }
    
        else{
            println!("The optional value is: {}",i);
            optional_value=Some(i+1); //Incrementing the value of i and setting it back to optional_value
        }
    }
}