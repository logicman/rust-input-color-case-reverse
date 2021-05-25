use colored::Colorize;
use rand::Rng;
use std::io;

/*
This program will take a word/sentence as an input and do the following on it:
- randomly upper and lower case characters
- randomly color the chars
- reverse the string
*/
fn main() {
    println!("Enter a word or a sentence --------->>>>>>");

    let mut word = String::new();
    let mut result = Vec::new();

    io::stdin()
        .read_line(&mut word)
        .expect("Failed to read line");

    //let result: String = word.chars().map(|c| c).collect();
    for c in word.chars() {
        // do something with character `c` and index `i`
        let colors = ["red", "green", "yellow", "blue", "magenta", "cyan"];
        let r = rand::thread_rng().gen_range(0..=5);
        let case = rand::thread_rng().gen_range(0..=1);
        if case == 1 {
            result.push(format!("{}", c.to_uppercase().to_string().color(colors[r])));
        } else {
            result.push(format!("{}", c.to_lowercase().to_string().color(colors[r])));
        }
    }
    result.reverse();
    println!("The output is: {}", result.concat());
}
