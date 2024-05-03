
//This gives Error
// fn main() {
//     let mut s1 = String::from("Hello");
//     update_word(&mut s1);
//     let s2 = &mut s1;
//     println!("{}", s1);
//     println!("{}", s2);
// }

//This will work
fn main() {
    let mut s1 = String::from("Hello");
    update_word(&mut s1);
    println!("{}", s1);
}

fn update_word(word: &mut String) {
    word.push_str(" World");
}