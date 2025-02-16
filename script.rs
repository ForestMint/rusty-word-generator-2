

use std::fs::File;
use std::io::{BufRead, BufReader};




fn generate_word(x: &str) -> &str {
    return "hello world";
}

fn pick_next_character(value : char) -> char{
    let c: char = 'a';
    c
}

fn parse_training_set() -> std::io::Result<()> {



    // Open the file for reading
    let file = File::open("training_set.txt")?;

    // Create a buffered reader to read the file
    let reader = BufReader::new(file);

    // Read the file line by line
    for line in reader.lines() {
        println!("{}", line?);
    }

    Ok(())

}

fn main () {
    //let foo = 10;
    //println!("The value of foo is {foo}"); 
    parse_training_set();
    generate_word("");
}

