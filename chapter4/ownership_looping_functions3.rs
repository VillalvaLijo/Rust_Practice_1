//ownership_looping_functions3.rs

//Sam Lijo

//Lord Alya Vijana

//program to practice looping, functions and ownership in rust

use std::io;
use std::any::type_name;

fn main(){

    let mut x = 5;

    println!("the value of i32 X before add_five function: {}", x);

    //add_five(&x);
    //This doesn't work because it is passing a imutable reference type, 
    //you need to pass it a mutable reference type, &mut x

    let xref = &mut x;

    add_five(xref);

    println!("the value of i32 X after add_five function: {}", x);

   
    println!("Input a floating point number: ");

    let mut user_input_float = String::new();

    io::stdin()
        .read_line(&mut user_input_float)
        .expect("Failed to read line");

    println!("Type of data you entered: {}", type_of(&user_input_float));

   
    let user_input_float: f64 = user_input_float.trim().parse().expect("Please type a number!");

    println!("Type of data after user input is changed: {}", type_of(&user_input_float));

    //let s1 = &mut user_input_float;

    //let s2 = add_five(s1);
    //can't do this, can't pass the function different types of number data like that

    //println!("User float after add_five function: {}", s2);

    //want to see what it would be like to add an integer to a f64 variable in RUST

    //let float_more = user_input_float + 5;
    //Rust doesn't allow you to add data types like that.

    //println!("Your float after you added five too it: {}", float_more);

    //let a: i8 = 3;
    //let b: i16 = 12;
    //let c: i64 = 100;
    //let d: i128 = 102;

    //println!("i8 plus i128: {}", a+d);
    //println!("i32 plus i64: {}", c+x);
    //println!("i8 plus i16: {}", a+b);
    //println!("i128+i64: {}", c+d);

    //These integer addition types don't work, you need to have the
    //same integer types in order to add them together.

    let mut y: i32 = 100;

    let y = add_hundred(&mut y);

    //println!("Value of y after adding a hundred: {}", y);

    println!("Type of y: {}", type_of(&y));

}

fn add_five(a: &mut i32){
     //a += 5;
    //can't use a+=5 on mutable reference type, you have to derefernece the reference 
    *a += 5;
  
}


fn type_of<T>(_: &T) ->&'static str {
    type_name::<T>()
    //:: <T> is an associated function of type_name that acts on
    //Type_name
}

fn add_hundred(a: &mut i32){

    for number in (0..10).rev(){
        *a += 10;
    }
}
