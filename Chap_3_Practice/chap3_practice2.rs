//chap3_practice2.rs

//Sam Lijo
//Lord Alya Vijana

//1/27/22

//Little function to practice palying around with control loops and data types

//use std::io;
use std::any::type_name;

fn main() {

    let tup: (f64, i32, u32) = (12.918257119, -23, 64);

    //for element in tup {
    //    println!("The value of this element is: {}", element);
    //}
    //Tuple's are not iterable so this code didn't work

    //for number in (0..2).rev(){
        //println!("The value of element {} in the tuple is: {}",number, tup.number);
    //    println!("The value of number is {}", number);
    //    println!("The data typf of number is {}", type_of(number));
    //}

    println!("First element in the Tuple: {}", tup.0);
    println!("Data type of first element in the tuple: {}", type_of(tup.0));
    println!("Data type of tup: {}", type_of(tup));

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    
    let mut count = 0;

    while count != 12{
        println!("{}", months[count]);
        count += 1;
    }

   
}

fn type_of<T>(_: T) ->&'static str {
    type_name::<T>()
    //:: <T> is an associated function of type_name that acts on
    //Type_name
}
