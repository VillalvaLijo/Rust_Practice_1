//ownership_looping_functions3.rs

//Sam Lijo

//Lord Alya Vijana

//program to practice looping, functions and ownership in rust

fn main(){

    let mut x = 5;

    println!("the value of i32 X before add_five function: {}", x);

    //add_five(&x);
    //This doesn't work because it is passing a imutable reference type, 
    //you need to pass it a mutable reference type, &mut x

    let xref = &mut x;

    add_five(xref);

    println!("the value of i32 X after add_five function: {}", x);

}

fn add_five(a: &mut i32){
     //a += 5;
    //can't use a+=5 on mutable reference type, you have to derefernece the reference 
    *a += 5;
  
}
