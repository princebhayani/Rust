fn main() {
    println!("Hello, world!");
    println!("Sum is:{}",do_sum(12,8));
}

fn do_sum(a: i32, b: i32) -> i32 {
	return a + b;
}