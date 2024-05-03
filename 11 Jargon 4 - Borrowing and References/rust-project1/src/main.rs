fn main() {
    let s1 = String::from("hello world");
    let s2 = &s1;

    println!("{}", s2);
    println!("{}", s1);
}
