//Ownership_lesson2.rs

//Sam Lijo

//Lord ALya Vijana

//little program to learn about ownership

fn main(){

    let s1 = gives_ownership();		// gives_ownership moves
					// value into s1
    let s2 = String::from("helo"0

    let s3 = takes_and_gives_back(s2);
}

fn gives_ownership() -> String{
    let some_string = String::from("yours");

    some_string
}

fn takes_and_gives_back(a_string: String) -> String{

    a_string
}

