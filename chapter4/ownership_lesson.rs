//ownership_lesson.rs

//This file is for practicing the rules of ownership in Rust that are being intoduced
//in Chapter 4 of the Rust Book

//Sam Lijo

//Lord Alya Vijana

fn main(){

    //let s = "hello";

    let mut s = String::from("hello");

    s.push_str(", world!"); //push_str() appends a literal to a string

    println!("{}", s); //this will print 'hello, world!'

}  //We are now outside of the scope of main, variables declared in main are no longer 
   // valide here.
