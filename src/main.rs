mod calc;

use calc::calc;
use std::io;
use std::io::prelude::*;

fn main() {
    println!("#============#");
    println!("# CALCULATOR #");
    println!("#============#");
    print!("\n");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                input = String::from(input.trim());
                let result = calc(&input);
                print!(">>> {}\n\n", result);
            }
            Err(_) => {
                println!("\nERROR: couldn't read line");
                continue;
            }
        }
    }
}
