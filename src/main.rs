extern crate linenoise;

mod lval;
mod ast;
mod env;

fn main() {
    let mut environment = env::Env::new();

    environment.add_val("1".to_string(), lval::LVal::Bool(true));
    environment.add_val("0".to_string(), lval::LVal::Bool(false));
    
     loop {
         let val = linenoise::input(">>> ");
         match val {
            None => { break }
            Some(input) => {
                linenoise::history_add(&input[..]);
                match ast::Ast::from_string(input){
                    Ok(a) => match lval::LVal::new(a, &environment) {
                        Ok(value) => println!("<<< {}", value.to_string()),
                        Err(e) => println!("!!! {}", e),
                            
                    },
                    Err(e) => println!("!!! {}", e),
                };
            }
        }
    }
}
