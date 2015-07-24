extern crate linenoise;

mod lval;
mod ast;
mod env;
use lval::specialform;

fn main() {
    let mut environment = env::Env::new();

    environment.add_val("1".to_string(), lval::LVal::Bool(true));
    environment.add_val("0".to_string(), lval::LVal::Bool(false));

    let def_form = specialform::SpecialForm::new(specialform::def_func, "def".to_string());
    environment.add_val("def".to_string(), lval::LVal::SpecialForm(def_form));
    
    loop {
        let val = linenoise::input(">>> ");
        match val {
            None => { break }
            Some(input) => {
                linenoise::history_add(&input[..]);
                match ast::Ast::new(input) {
                    Ok(a) => println!("<<< {} ", lval::LVal::new(a, &mut environment).to_string() ),
                    Err(e) => println!("!!! {}", e),
                }                
            },
        }
    }
}
