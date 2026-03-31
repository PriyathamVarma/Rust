
# 🦀 Rust Programming – Basics

## 1. 🚀 Why Learn Rust?

Rust is one of the fastest-growing programming languages in the world.

### Key Advantages:
- 📈 Fast-growing language in the tech industry  
- 🔄 Makes it easier to learn other languages (Java, Python, C++, C#)  
- ⚡ Very high performance (similar to C/C++)  
- 🧠 Uses less memory compared to many languages  
- 🛠️ Used for:
  - Web servers 🌐  
  - Game development 🎮  
  - Operating systems 💻  
  - System-level programming  

💡 **Simple Idea:**  
Rust provides both **speed and safety**.

---

## 2. ⚔️ Difference Between Rust and C/C++

### Core Difference:

| Feature            | Rust 🦀                  | C / C++ ⚙️              |
|------------------|------------------------|------------------------|
| Memory Safety     | ✅ Automatic (compiler checks) | ❌ Manual management |
| Bugs (crash/leak) | Less likely            | More common            |
| Ease of Use       | Safer                  | Requires careful handling |

### 🧠 Key Concept:
- Rust is **"safe by default"**
- Prevents:
  - Memory leaks  
  - Crashes  
  - Unsafe memory access  

💡 **Analogy:**  
- C/C++ → Driving without safety features 🚗  
- Rust → Driving with automatic safety systems 🛡️  

---

## 3. 🧩 Basic Rust Program

```rust
fn main() {
    println!("Hello, world!");
}
````

### 🔍 Explanation:

* `fn` → defines a function
* `main()` → entry point of the program
* `println!` → prints output to the console

💡 **Important Rule:**
Every Rust program must have a `main()` function.

---

## 4. 🔁 Functions in Rust

### Example 1:

```rust
fn main() {
    test();
}

fn test() {
    println!("Hello, world!");
    println!("Hello, world!");
    println!("Hello, world!");
    println!("Hello, world!");
}
```

### Explanation:

* `main()` calls the `test()` function
* `test()` executes multiple print statements

---

### Example 2 (Calling Multiple Times):

```rust
fn test() {
    println!("Hello, world!");
    println!("Hello, world!");
    println!("Hello, world!");
    println!("Hello, world!");
}

fn main() {
    test();
    test();
}
```

### 🔍 Key Points:

* Functions can be called multiple times
* Execution always starts from `main()`
* Other functions run only when called

💡 **Analogy:**

* `main()` = Manager 👨‍💼
* `test()` = Worker 👷

---

## 5. 💬 Comments in Rust

Comments are used to:

* Explain code 📝
* Improve readability 👀
* Disable code during testing 🚫

---

### 5.1 Single-line Comments

```rust
// This is a comment
println!("Hello");
```

* Starts with `//`
* Ignored by the compiler

---

### 5.2 Multi-line Comments

```rust
/*
This is a multi-line comment
It can span multiple lines
*/
```

* Starts with `/*`
* Ends with `*/`

---

## 🧠 Quick Revision

* Rust = Fast + Safe language
* `main()` = Starting point
* Functions are reusable 🔁
* Comments improve understanding
* Rust automatically manages memory ✅

