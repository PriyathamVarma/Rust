# 🦀 Rust Data Structures

## 1. 📌 What are Data Structures?

Data structures are used to **store and organize data** in a program.

💡 Different data structures are used for different purposes.

---

## 2. 📦 Common Data Structures in Rust

- Array  
- Vector (`Vec`)  
- Tuple  
- HashMap  

---

## 3. 📚 Arrays

An array is a **fixed-size collection** of elements of the **same type**.

### 🔑 Features:
- Fixed size ❌ (cannot grow or shrink)  
- Same data type only  
- Indexed access  

---

### Example:

```rust
fn main() {
    let fruits = ["apple", "banana", "orange"];

    println!("Last fruit: {}", fruits[2]);
}
````

### 🔍 Explanation:

* Index starts from `0`
* `fruits[2]` → "orange"

---

## 4. 📈 Vectors (`Vec`)

A vector is a **dynamic (resizable) array**.

### 🔑 Features:

* Can grow or shrink ✅
* Same data type
* More flexible than arrays

---

### Example:

```rust id="7aqk7m"
fn main() {
    let mut fruits = vec!["apple", "banana"];

    fruits.push("cherry");

    println!("Last fruit: {}", fruits[2]);
}
```

---

## 5. 🧩 Tuples

A tuple stores **multiple values of different types**.

### 🔑 Features:

* Can hold mixed data types
* Fixed size
* Access using index with dot (`.`)

---

### Example:

```rust id="r4tqnt"
fn main() {
    let person = ("John", 30, true);

    println!("Name: {}", person.0);
    println!("Age: {}", person.1);
    println!("Is active: {}", person.2);
}
```

---

## 6. 🗺️ HashMap

A HashMap stores **key-value pairs**.

### 🔑 Features:

* Lookup values using keys 🔑
* Dynamic size
* Useful for mapping data

---

### Example:

```rust id="q7d02y"
use std::collections::HashMap;

fn main() {
    let mut capital_cities = HashMap::new();

    capital_cities.insert("France", "Paris");
    capital_cities.insert("Japan", "Tokyo");

    println!("Capital of Japan is {}", capital_cities["Japan"]);
}
```

---

## 7. 📊 Data Structures Overview

| Data Structure | Use Case                       | Can Grow? |
| -------------- | ------------------------------ | --------- |
| Array          | Fixed-size list (same type)    | ❌ No      |
| Vector (Vec)   | Dynamic list (same type)       | ✅ Yes     |
| Tuple          | Group different types together | ❌ No      |
| HashMap        | Key-value pair storage         | ✅ Yes     |

---

## 🧠 Quick Revision

* Array → fixed size 📚
* Vector → dynamic size 📈
* Tuple → mixed data types 🧩
* HashMap → key-value pairs 🗺️
