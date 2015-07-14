use std::string;
use std::vec;

pub enum Ast{
    Token(string::String),
    SubList(vec::Vec<Ast>),
}

impl Ast{
    pub fn to_string(self: Ast) -> string::String {
        self.to_string_with_indent(0)
    }

    fn to_string_with_indent(self: Ast, indent: i32) -> string::String {
        let mut output = "".to_string();
        
        match self{
            Ast::Token(t) => output.push_str(&t[..]),
            Ast::SubList(v) => {
                output.push_str("\n");
                for _ in 0..indent{
                    output.push_str(" ");
                }
                output.push_str("(");
                for element in v {
                    output.push_str(&element.to_string_with_indent(indent+1)[..]);
                }
                output.push_str(")");
            },
        };

        output.push_str(" ");
        return output;
    }
}
