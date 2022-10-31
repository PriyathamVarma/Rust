# Rust
Rust programming


> Learn rust in a easy way

## Fundamentals

### Cargo

Cargo is a package manager for rust.

Creating a new folder

```
cargo new first

```

This will create a folder --> src --> main.rs(a basic rust file)
and
Cargo.toml which is similar to package.json

Running the code inside the folder
 
```
cargo run

```

You will get something like 

```
Compiling first v0.1.0 (C:\Users\......)
    Finished dev [unoptimized + debuginfo] target(s) in 1.68s
     Running `target\debug\first.exe`
Hello, world!

```


### Variables

* Statically-typed language

```

let number: i32 = 4;

//name type value

```

YOu can also have multiple variables at once

```
fn main() {
    let (names,titles): (i32,i32) = (10,20);
    println!("{}",names);
}

```

We can also use multiple println!

```

fn main() {
    let (names,titles): (i32,i32) = (10,20);
    println!("{}",names);

    let many: (i32,i32) = (100,20);
    println!("{}",many.0);
}

```

## Useful Links


1. [creates.io](https://crates.io/)

