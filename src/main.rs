mod ast;

fn main() {
    //let a = ast::Ast::SubList(vec![ast::Ast::Token("Hello".to_string()),
    //                               ast::Ast::SubList(vec![ast::Ast::Token("Wo".to_string()),ast::Ast::Token("rl".to_string()), ast::Ast::Token("d!".to_string())])]);
    match ast::Ast::from_string("(Hello ( world! ) (and any (other worlds! )) )".to_string()){
        Ok(a) => println!("{}", a.to_string()),
        Err(e) => println!("{}", e),
    };    
}
