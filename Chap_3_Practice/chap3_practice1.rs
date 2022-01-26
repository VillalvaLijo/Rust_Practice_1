//Practice File to Practice COntrol loops and initializing data types and 
//other little things from chapter 3 of the learning RUST book

//Samuel Villalva Lijo
//Lord Alya Vijana
//1/26/22

use std::any::type_name;

fn main(){
    let x: i32 = 78;

    let y: f32 = 12.89156;

    println!("The value of x is: {}", x);
    println!("The data type of x is: {}", type_of(x));
    println!("The value of y is: {}", y);
    println!("The data type of y is: {}", type_of(y));
}

fn type_of<T>(_: T) ->&'static str {
    type_name::<T>()
}
