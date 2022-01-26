//Practice File to Practice COntrol loops and initializing data types and 
//other little things from chapter 3 of the learning RUST book

//Samuel Villalva Lijo
//Lord Alya Vijana
//1/26/22

use std::any::type_name;
use std::io;

//use rand::Rng;
//can't use Rand because it is a create that needs to be added to the 
// .toml file

fn main(){
    let x: i32 = 78;

    let y: f32 = 12.89156;

    //let z = rand::thread_rng().gen_range(1..10_000);

    println!("The value of x is: {}", x);
    println!("The data type of x is: {}", type_of(x));
    println!("The value of y is: {}", y);
    println!("The data type of y is: {}", type_of(y));

    //println!("The value of the random number z is: {}", z);

    println!("Input a floating point number: ");

    let mut user_input_float = String::new();

    io::stdin()
        .read_line(&mut user_input_float)
        .expect("Failed to read line");

    println!("You entered: {}", user_input_float);
    println!("Type of what you entered: {}", type_of(user_input_float));
}

fn type_of<T>(_: T) ->&'static str {
    type_name::<T>()
    //:: <T> is an associated function of type_name that acts on
    //Type_name
}
