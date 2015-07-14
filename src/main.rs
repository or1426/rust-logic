mod ast;

fn main() {
    let a = ast::Ast::SubList(vec![ast::Ast::Token("Hello".to_string()),
                                   ast::Ast::SubList(vec![ast::Ast::Token("Wo".to_string()),ast::Ast::Token("rl".to_string()), ast::Ast::Token("d!".to_string())])]);
    println!("{}", a.to_string());
}
