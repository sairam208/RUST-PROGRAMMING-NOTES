fn main() {
    //for loop example
    println!("For loop using range 1..7");
    for i in 1..8{ //1..8 means 1 to 7 and if 1..=8 means 1 to 8
        print!("{i}");
    }
    
    //Different way to write the for loop
    //Using vectors,vector is a growable array type in Rust
    print!("\n");
    println!("For loop Using the vector");
    let numbers = vec![1, 2, 3, 4, 5, 6, 7];
    for n in numbers {
        print!("{n}");
    }
    print!("\n");
    //Using list 
    println!("For loop using list");
    //The difference b2w list and array is that list is fixed size and array is dynamic size
    // array is also fixed size in rust but we can use vectors for dynamic size
    //Vectors are heap allocated while arrays are stack allocated
    //vectors can grow and shrink in size while arrays have a fixed size
    let list=[1,2,3,4,5,6,7];
    for i in list{
        print!("{}",i);
    }



    println!("\n");
    println!("While loop example:");
    //While loop 
    let mut i:i32=10; //mut keyword is to make the var i as muttable if i is not declared as mut like let i:i32=10 then i value cnt be changed
    while i>0 {
        print!("{},",i);
        i-=1;
    }
    //Looping through array using while loop
    println!("\n");
    println!("While loop through array:");
    let arr=[10,20,30,40,50];
    let mut index=0;
    while index < arr.len(){
        print!("{},",arr[index]);
        index+=1;   
    }
    //While loop using vectors
    let vector=vec![11,22,33,44,55];
    println!("\n");
    println!("While loop through vector:");
    let mut idx=0;
    while idx < vector.len(){
        print!("{},",vector[idx]);
        idx+=1;
    }
    //Looping through array using loop keyword
    println!("\n");
    println!("Looping through array using loop keyword:");
    let arr2=[100,200,300,400,500];
    let mut idx=0;
    loop {  //Loop keyword creates an infinite loop unless explicitly broken out of same like while true in other languages,in rust we cant use while true
        if idx >= arr2.len(){
            break;
        }
        print!("{},",arr2[idx]);
        idx+=1;
    }
}    