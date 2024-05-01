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

##### 2. Booleans
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

##### 3. String
- There are two ways of doing ```strings``` in rust. We’ll be focussing on the easier one.
```
fn main() {
    let greeting = String::from("hello world");
    println!("{}", greeting);

    // print!("{}", greeting.chars().nth(1000))
}
```
---

## 05 Conditionals, loops
##### Conditionals
```
pub fn main() {
    let x = 99;
    let is_even = is_even(x);
    if is_even {
        print!("{} is even", x);
    } else {
        print!("{} is odd", x);
    }
}

pub fn is_even(x: i32) -> bool {
    return x % 2 == 0;
}
```

##### Loops
```
pub fn main() {
    let str = String::from("Prince Bhayani");
    println!("First name {}", get_first_name(str))
    
}

pub fn get_first_name(str: String) -> String {
    let mut first_name = String::from("");
    for c in str.chars() {
        if c == ' ' {
            break
        }
        first_name.push(c);
    }
    return first_name;
}
```
---

## 06 Functions
```
fn do_sum(a: i32, b: i32) -> i32 {
	return a + b;
}
```
---

## 07 Memory Management in Rust
![](./07%20Memory%20Managment%20in%20Rust/pic1.png)
- Whenever you run a program (C++, Rust, JS), it ```allocates ```and ```deallocates``` memory on the RAM.

For example, for the following JS code:
```
function main() {
  runLoop();
}

function runLoop() {
  let x = [];
  for (let i = 0; i < 100000; i++) {
    x.push(1);
  }
  console.log(x);
}

main();
```
> as the ```runLoop``` function runs, a new array is pushed to RAM, and eventually ```garbage collected```

There are 3 popular ways of doing memory management.
![](./07%20Memory%20Managment%20in%20Rust/pic2.png)

- Memory management is a crucial aspect of programming in Rust, designed to ensure safety and efficiency without the need for a garbage collector. 
- Not having a ```garbage collector``` is one of the key reasons rust is so fast
- It achieves this using the
    1. Mutability
    2. Heap and memory
    3. Ownership model
    4. Borrowing and references
    5. Lifetimes

---

## 08 Jargon #1 - Mutability
##### Mutability
Immutable ```variables``` represent variables whose value cant be changed once assigned.
```
fn main() {
    let x: i32 = 1;
    x = 2; // Error because x is immutable
    println!("{}", x);
}
```
> [!NOTE]
> By default, all variables in Rust are immutable.

1. Immutable data is inherently thread-safe because if no thread can alter the data, then no synchronization is needed when data is accessed concurrently.
2. Knowing that certain data will not change allows the compiler to optimize code better. 

- You can make variables mutable by using the ```mut``` keyword.
```
fn main() {
    let mut x: i32 = 1;
    x = 2; // No error
    println!("{}", x);
}
```
> const in Javascript is not the same as immutable variables in rust. In JS, you can still update the contents of const ```arrays``` and ```objects ```.