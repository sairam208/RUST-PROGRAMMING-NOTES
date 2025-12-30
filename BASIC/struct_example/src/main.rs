fn main() {
    //Defining structs
    struct Student{
        name:String,
        age:u8,
        class:String,
        grade:char,
    }
    //Declaring a struct and acessing it
    let student1=Student{name:"sairam".to_string(),age:19,class:"ECE_C".to_string(),grade:'O'};
    //Accessing struct fields
    println!("Student Name: {}",student1.name);
    println!("Student Age: {}",student1.age);
    println!("Student Class: {}",student1.class);
    println!("Student Grade: {}",student1.grade);
    //Creating another struct instance with some data from the first struct
    let student2=Student{name:student1.name.clone(), //Cloning the name field to avoid ownership issues
        age:23, //New age
        ..student1}; //Using .. syntax to copy remaining fields from student1
    println!("Student2 Name: {}",student2.name);
    println!("Student2 Age: {}",student2.age);
    println!("Student2 Class: {}",student2.class);
    println!("Student2 Grade: {}",student2.grade);
    //Tuple structs
    struct Color(u8,u8,u8); //Defining a tuple struct Color with 3 u8 fields
    let black=Color(0,255,255); //Creating an instance of Color
    println!(" Color RGB: ({},{},{})",black.0,black.1,black.2); //Accessing tuple struct fields using dot notation
    //Unit-like structs
    struct Marker; //Defining a unit-like struct Marker
    let _marker=Marker; //Creating an instance of Marker
    println!("Unit-like struct Marker created");
    //Use of unit structs is mainly for type safety and to implement traits(traits means ) without any data
    //Struct destructuring
    let Student{name,age,class,grade}=student2; //Destructuring the student2 struct
    println!("Destructured Student2 Name: {}",name);
    println!("Destructured Student2 Age: {}",age);
    println!("Destructured Student2 Class: {}",class);
    println!("Destructured Student2 Grade: {}",grade);
}
