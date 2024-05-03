struct User {
    active :bool,
    sign_in_count:u64,
}

fn main(){
    let mut user1 = User{
        active:true,
        sign_in_count:1
    };

    print_name(user1);
    
    // this line gives error
    print!("User 1 username: {}", user1.active);
}

fn print_name(user1: User){
    print!("User 1 username: {}",user1.active);
}