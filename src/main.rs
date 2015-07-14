extern crate linenoise;
use std::string;
mod ast;

fn main() {
     loop {
         let val = linenoise::input(">>> ");
        match val {
            None => { break }
            Some(input) => {
                linenoise::history_add(&input[..]);
                match ast::Ast::from_string(input){
                    Ok(a) => println!("<<< {}", a.to_string()),
                    Err(e) => println!("!!! {}", e),
                };
            }
        }
    }
}
