# 🦀 Rust Variables

## 1. 📦 What are Variables?

Variables are **containers used to store data values**, such as:
- Numbers 🔢  
- Text (strings) 🔤  

---

## 2. 🛠️ Creating Variables in Rust

To create a variable, use the `let` keyword.

### Example:

```rust
fn main() {
    let name = "John";
    println!("My first name is: {}", name);
}
````

### 🔍 Explanation:

* `let` → used to declare a variable
* `name` → variable name
* `"John"` → value stored in the variable

---

## 3. 🔄 What is `{}` in Rust?

* `{}` is a **placeholder** used in `println!()`
* It gets replaced by variable values

### Example:

```rust
fn main() {
    let name = "John";
    println!("My first name is: {}", name);
}
```

### Output:

```
My first name is: John
```

---

## 4. 🧩 Multiple Placeholders

You can use multiple `{}` placeholders.

### Example:

```rust
fn main() {
    let name = "John";
    let age = 30;
    println!("{} is {} years old.", name, age);
}
```

### Output:

```
John is 30 years old.
```

---

## 5. 🔢 Order of Placeholders (Important ⚠️)

* Values are filled **in order** of `{}`

### Example (Correct Order):

```rust
println!("{} is {} years old.", name, age);
```

* First `{}` → `name`
* Second `{}` → `age`

---

### ❌ Wrong Order Example:

```rust
fn main() {
    let name = "John";
    let age = 30;
    println!("{} is {} years old.", age, name);
}
```

### Output:

```
30 is John years old.
```

💡 **Key Point:**
Order of variables must match placeholders.

---

## 6. 🔒 Immutable Variables (Default Behavior)

By default, variables in Rust are **immutable** (cannot be changed).

### Example:

```rust
fn main() {
    let x = 5;
    x = 10; // ❌ Error
    println!("{}", x);
}
```

### ❗ Why?

Rust prevents accidental changes → improves safety.

---

## 7. 🔓 Mutable Variables (`mut` keyword)

To allow changing a variable, use `mut`.

### Example:

```rust
fn main() {
    let mut x = 5;
    println!("Before: {}", x);

    x = 10;

    println!("After: {}", x);
}
```

### Output:

```
Before: 5
After: 10
```

### 🔍 Explanation:

* `mut` → makes variable **mutable (changeable)**

---

## 🧠 Quick Revision

* `let` → declare variable
* `{}` → placeholder in printing
* Order of placeholders matters ⚠️
* Variables are **immutable by default** 🔒
* Use `mut` to make them **mutable** 🔓

