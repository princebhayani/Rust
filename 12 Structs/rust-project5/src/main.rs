#[derive(Clone)]
struct User {
    active: bool,
    sign_in_count: u64,
    username: String,
}

fn main() {
    let mut user1 = User {
        active: true,
        sign_in_count: 1,
        username: "harkirat".to_string()
    };

    change_name(user1.clone());
    print!("User 1 username: {}", user1.active); // Error - can not use borrowed value
}

fn change_name(user1: User) {
    print!("User 1 username: {:?}", user1.active);
}