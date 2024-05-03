fn main() {
    let user1 = User{
        active:true,
        username: String::from("username1"),
        email:String::from("example@gmail.com"),
        sign_in_count:1,
    };
    println!("{}",user1.username);
}

struct User{
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}