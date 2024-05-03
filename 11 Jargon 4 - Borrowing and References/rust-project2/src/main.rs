fn main() {
    let my_string = String::from("Hello world");
    borrowing_ownership(&my_string);
    println!("{}",my_string);
}

fn borrowing_ownership(some_string: &String){
    println!("{}",some_string);
}