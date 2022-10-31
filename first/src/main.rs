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

