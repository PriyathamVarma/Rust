// Use cargo run to run the file

// Function main which prints some text
fn main() {
    let (names,titles): (i32,i32) = (10,20);
    println!("{}",names);

    let many: (i32,i32) = (100,20);
    println!("{}",many.0);

    // Mutable variables
    let interest = "myself";
    interest = "change";
    println!("{}",interest);
}

