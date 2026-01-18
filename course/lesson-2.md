# Vectors and arrays


code:

#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

fn main() {

    // Variable to store struct
    let my_deck:Deck = Deck{cards: vec![]};
    println!("Heres your deck: {}", my_deck);
}


error:
`Deck` doesn't implement `std::fmt::Display`
in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) insteadrustcClick for full compiler diagnostic
macros.rs(143, 28): Actual error occurred here
macros.rs(143, 28): Error originated from macro call here
main.rs(10, 5): Error originated from macro call here
main.rs(10, 32): required by this formatting parameter
main.rs(2, 1): the trait `std::fmt::Display` is not implemented for `Deck`


solution 1:
println!("Heres your deck: {:?}", my_deck);


then 

#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

fn main() {

    // Variable to store struct
    let my_deck:Deck = Deck{cards: vec![]};
    println!("Heres your deck: {:?}", my_deck);
}


using format, loop and arrays:


#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

fn main() {
    // ARRAYS
    // List of suites - hearts, diamnonds, clubs, spades
    let suites = ["Hearts", "Diamonds", "Clubs", "Spades"];
    // List of values - 2-10, J, Q, K,A
    let values = [
        "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A",
    ];

    // Loop
    for suite in suites {
        for value in &values {
            println!("{} of {}", value, suite);
        }
    }


    // Variable to store struct
    let my_deck:Deck = Deck{cards: vec![]};
    println!("Heres your deck: {:?}", my_deck);
}



## Creating cards

#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

fn main() {
    // ARRAYS
    // List of suites - hearts, diamnonds, clubs, spades
    let suites = ["Hearts", "Diamonds", "Clubs", "Spades"];
    // List of values - 2-10, J, Q, K,A
    let values = [
        "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A",
    ];

    // Loop
    for suite in suites {
        for value in &values {
            println!("{} of {}", value, suite);

            let card = format!("{} of {}", value, suite);
            my_deck.cards.push(card);
        }
    }


    // Variable to store struct
    let my_deck:Deck = Deck{cards: vec![]};
    println!("Heres your deck: {:?}", my_deck);
}



