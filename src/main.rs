extern crate linenoise;

mod lval;
mod ast;
mod env;
use lval::specialform;
use lval::lfunc;
use lval::ltype;

fn main() {
    let mut environment = env::Env::new();

    environment.add_val("1".to_string(), lval::LVal::Bool(true));
    environment.add_val("0".to_string(), lval::LVal::Bool(false));

    environment.add_val("p".to_string(), lval::LVal::SpecialForm(specialform::SpecialForm::new(specialform::placeholder_fn, "p".to_string())));
    let e_cpy = environment.clone();
    environment.add_val("def".to_string(), lval::LVal::Func(lfunc::LFunc::Builtin(lfunc::BuiltinFn::new(lfunc::def_fn, vec![ltype::LType::PlaceHolder, ltype::LType::Unknown], ltype::LType::Unknown, &e_cpy))));
    
    loop {
        let val = linenoise::input(">>> ");
        match val {
            None => { break }
            Some(input) => {
                linenoise::history_add(&input[..]);
                match ast::Ast::new(input) {
                    Ok(a) => println!("<<< {} ", lval::LVal::new(a, &environment).eval(&mut environment).to_string()),
                    Err(e) => println!("!!! {}", e),
                }                
            },
        }
    }
}
