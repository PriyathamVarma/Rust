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

* For mutable data
wrong code
```
fn main() {

    // Mutable variables
    let interest = "myself";
    interest = "change";
    println!("{}",interest);
}
```
Right code
```
fn main() {

    // Mutable variables
    let mut interest = "myself";
    interest = "change";
    println!("{}",interest);
}
```


* Constant variables

```
// Use cargo run to run the file

// Function main which prints some text
fn main() {

    // Constants
    // const UPPER_CASE : <data-type> = some constant
    const ROLL_NO : i32 = 56;
    println!("{}",ROLL_NO);
}

```

* Scope

```
fn main() {
    
    // Scope

    let x = 10;
    {
        let y = 5;
        println!("X:{},Y:{}",x,y);// No error
    }

    println!("X:{},Y:{}",x,y);// error because Y is block scope

}
```

* Scope for same variable

```
// Use cargo run to run the file

// Function main which prints some text
fn main() {
    
    // Scope for same variable

    let x = 10;
    {
        let x = 5;
        println!("X:{}",x);// x is 5
    }

    println!("X:{}",x);// x is 10

}


```

## Useful Links


1. [creates.io](https://crates.io/)

