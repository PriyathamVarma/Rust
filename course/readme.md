# Rust Programming – Beginner Notes

---

## Installation

Official website:  
https://www.rust-lang.org/tools/install

### Install Rust using `rustup`

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
````

### Verify Installation

Check if Cargo is installed:

```bash
cargo
```

Check Cargo version:

```bash
cargo --version
```

---

## Creating and Running Rust Projects

Rust projects are managed using **Cargo**, which handles:

* Project creation
* Dependency management
* Building
* Running

---

## Example Project: Card Deck Game in Rust

We will conceptually build a **card deck** with basic operations.

### Core Functions (Conceptual Design)

* `new()`
  Creates a new deck containing a list of playing cards

* `shuffle()`
  Shuffles the order of cards in the deck

* `deal()`
  Removes some playing cards from the deck and returns them as a new list

This structure helps understand:

* Structs
* Methods
* Ownership and borrowing
* Vectors (`Vec<T>`)

---

## Cargo (Rust Package Manager)

Cargo is similar to **npm** in the Node.js ecosystem.

### Create a New Project

```bash
cargo new <project_name>
```

Example:

```bash
cargo new card_deck
```

This creates:

* `Cargo.toml` → Project configuration
* `src/main.rs` → Entry point

---

### Run a Rust Project

```bash
cargo run
```

Run quietly (no extra build output):

```bash
cargo run -q
```

---

## Summary

* Rust is installed and managed using `rustup`
* `cargo` is the package manager and build tool
* Projects are created using `cargo new`
* Code is executed using `cargo run`
* Card deck example introduces core Rust concepts in a practical way

---
