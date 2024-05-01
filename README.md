# Rust

- learning rust from the basics.
- It covers concepts like memory management, ownership, borrowing, structures, data types, enums, and much more.

---

## 01 Preface
### Phrase #1
- In Rust, if you program compiles, it probably works.

### Phrase #2
- You can't segfault if you don't have null.

### Phrase #3
- Rust doesn't hide complexity from developers it offers them the right tools to manage all the complexity.

---

## 02 Why rust? Isn’t Node.js enough?
### Type safety
### Systems language
It is intended to be used (but not restricted to) to do lower level things
1. Building a Compiler
2. Building a browser
3. Working closer to the OS/kernel

### Generally faster
- Rust has a separate compilation step (similar to C++) that spits out an optimized binary and does a lot of static analysis at compile time. 
- JS does JIT compilation. 

![](/02%20Why%20Rust/pic1.png)
![](/02%20Why%20Rust/pic%202.png)

### Concurrency
- Rust has built-in support for concurrent programming allowing multiple threads to perform tasks simultaneously without risking data races.
- Javascript is single threaded generally (there are some projects that tried making it multi threaded but rarely used).

### Memory safe
- Rust has a concept of owners, borrowing and lifetimes that make it extremely memory safe.

> [!IMPORTANT]
> Rust doesn't hide complexity from developers it offers them the right tools to manage all the complexity.

---

- Rust projects can be used to do a lot of things
    1. Create Backend for a Full stack app
    2. Create CLIs (command line interfaces)
    3. Create browsers
    4. Great Code Editors

- For running rust locally, you should be able to run ```cargo``` in your terminal.
> [!NOTE]
> Cargo is similar to ```npm```. It’s a package manager for rust.

---
#### 03 Initializing a rust project
- Run the following command to bootstrap a simple Rust project

```Rust
mkdir rust-project
cd rust-project
cargo init
```
---

## 04 Simple Variables in Rust
- You can define variables using the ```let``` keyword (very similar to JS)
- You can assign the type of the variable, or it can be inferred as well.
##### 1. Numbers
```
fn main() {
    let x: i32 = 1;
    println!("{}", x);
}
```
- Here, i32 shows size(in bits) of variable.
- i8, i16, i32, i64 etc for signed integer and u8,u16, u32, u64 is for unsigned integer.
- for float f8, f16,f32 etc.
- by default it will i32. also if you not assign type it will assign by default.

> [!CAUTION]
> What happens if we overflow?
```
fn main() {
    let mut num: i8 = 124;
    for i in 0..100 {
        num += 127;
    }
    print!("Number: {}", num)
}
```
output:
> attempt to add with overflow

##### 02 Booleans
- Booleans can have two states, true or false.
```
fn main() {
    let is_male = false;
    let is_above_18 = true;
    
    if is_male {
        println!("You are a male");

    } else {
        println!("You are not a male");
    }

    if is_male && is_above_18 {
        print!("You are a legal male");
    }
}
```

##### 03 String
- There are two ways of doing ```strings``` in rust. We’ll be focussing on the easier one.
```
fn main() {
    let greeting = String::from("hello world");
    println!("{}", greeting);

    // print!("{}", greeting.chars().nth(1000))
}
```