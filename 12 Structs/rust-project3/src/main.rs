#[derive(Copy, Clone)]
struct User {
    active: bool,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        active: true,
        sign_in_count: 1,
    };

    print_name(user1);
    println!("User 1 username in main: {}", user1.active); // Error goes away because user1 is copied
}

fn print_name(user1: User) {
    println!("User 1 username: {}", user1.active);
}