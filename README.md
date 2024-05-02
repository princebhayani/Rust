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
```Rust
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
```Rust
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
```Rust
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
```Rust
fn main() {
    let greeting = String::from("hello world");
    println!("{}", greeting);

    // print!("{}", greeting.chars().nth(1000))
}
```
---

## 05 Conditionals, loops
##### Conditionals
```Rust
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
```Rust
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
```Rust
fn do_sum(a: i32, b: i32) -> i32 {
	return a + b;
}
```
---

## 07 Memory Management in Rust
![](./07%20Memory%20Managment%20in%20Rust/pic1.png)
- Whenever you run a program (C++, Rust, JS), it ```allocates ```and ```deallocates``` memory on the RAM.

For example, for the following JS code:
```Rust
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
```Rust
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
```Rust
fn main() {
    let mut x: i32 = 1;
    x = 2; // No error
    println!("{}", x);
}
```
> const in Javascript is not the same as immutable variables in rust. In JS, you can still update the contents of const ```arrays``` and ```objects ```.

---

## 09 Jargon #2 - Stack vs Heap
##### Stack vs. Heap Allocation
Rust has clear rules about stack and heap data management:

- Stack: Fast allocation and deallocation. Rust uses the stack for most primitive data types and for data where the size is known at compile time (eg: numbers).
- Heap: Used for data that can grow at runtime, such as vectors or strings.

##### What’s stored on the stack?
1. Numbers - i32, i64, f64 …
2. Booleans - true, false
3. Fixed sized arrays 

##### What’s stored on heap?
1. Strings
2. Vectors 

##### Examples
- Hello world with numbers
![](./09%20Jargon%202%20-%20Stack%20vs%20Heap/pic1.png)

- Hello world with functions
![](./09%20Jargon%202%20-%20Stack%20vs%20Heap/pic2.png)
![](./09%20Jargon%202%20-%20Stack%20vs%20Heap/pic3.png)
![](./09%20Jargon%202%20-%20Stack%20vs%20Heap/pic4.png)

- Hello world with strings
![](./09%20Jargon%202%20-%20Stack%20vs%20Heap/pic5.png)

##### Memory in action
```Rust
fn main() {
    stack_fn();   // Call the function that uses stack memory
    heap_fn();    // Call the function that uses heap memory
    update_string();  // Call the function that changes size of variable at runtime
}

fn stack_fn() {
    // Declare a few integers on the stack
    let a = 10;
    let b = 20;
    let c = a + b;
    println!("Stack function: The sum of {} and {} is {}", a, b, c);
}

fn heap_fn() {
    // Create a string, which is allocated on the heap
    let s1 = String::from("Hello");
    let s2 = String::from("World");
    let combined = format!("{} {}", s1, s2);
    println!("Heap function: Combined string is '{}'", combined);
}

fn update_string() {
    // Start with a base string on the heap
    let mut s = String::from("Initial string");
    println!("Before update: {}", s);
    println!("Capacity: {}, Length: {}, pointer: {:p}",s.capacity(),s.len(),s.as_ptr());

    // Append some text to the string
    s.push_str(" and some additional text");
    println!("After update: {}", s);
    println!("Capacity: {}, Length: {}, pointer: {:p}",s.capacity(),s.len(),s.as_ptr());
}
```

Output:
```
Stack function: The sum of 10 and 20 is 30
Heap function: Combined string is 'Hello World'
Before update: Initial string
Capacity: 14, Length: 14, pointer: 0x1cc97809940
After update: Initial string and some additional text
Capacity: 39, Length: 39, pointer: 0x1cc97802b90
```

Memory stack and heap:
![](./09%20Jargon%202%20-%20Stack%20vs%20Heap/pic61.png)
![](./09%20Jargon%202%20-%20Stack%20vs%20Heap/pic62.png)

---