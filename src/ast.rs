use std::string;
use std::vec;
use std::result;

pub enum Ast{
    Token(string::String),
    SubList(vec::Vec<Ast>),
    SubArray(vec::Vec<Ast>),
}

impl Clone for Ast{
    fn clone(&self)->Ast{
        match self {
            &Ast::Token(ref s) => Ast::Token(s.clone()),
            &Ast::SubList(ref v) => Ast::SubList(v.clone()),
            &Ast::SubArray(ref v) => Ast::SubList(v.clone()),
        }
    }
}

impl Ast{
    pub fn new(s: string::String) -> result::Result<Ast, &'static str> {
        //First add spaces to ensure brackets end up as tokens by themselves
        let mut s = s.replace("(", " ( ");
        s = s.replace(")", " ) ");
        s = s.replace("[", " [ ");
        s = s.replace("]", " ] ");
        let str = s.trim();
        let mut v: vec::Vec<string::String> = str.split(" ").map(|s| s.to_string()).collect();

        v.reverse();

        match v.pop() {
            None => {return Err("I need some tokens blad")},
            Some(token) => match &token[..] {
                "["|"(" => (),
                _ => {return Ok(Ast::Token(token));},
            }
            
        };
        
        match Ast::from_string_vec(v) {
            Ok((_, ast)) => Ok(ast),
            Err(e) => Err(e),
        }
    }

    fn from_string_vec(mut v : vec::Vec<string::String>) -> result::Result<(vec::Vec<string::String>, Ast), &'static str> {
        let mut ast_vec = vec![];

        loop{
            match v.pop() {
                None => break,
                Some(str) => match &str[..] {
                    ")" => return Ok((v, Ast::SubList(ast_vec))),
                    "]" => return Ok((v, Ast::SubArray(ast_vec))),
                    "(" | "[" => {
                        match Ast::from_string_vec(v.clone()) {
                            Ok((v_ret, ast)) => {
                                ast_vec.push(ast);
                                v = v_ret;
                            },
                            Err(s) => return Err(s),
                        }
                    },
                    "" => (),
                    " " => (), 
                    _ => {
                        //println!("Debug: {}", str);
                        ast_vec.push(Ast::Token(str.to_string()))
                    },
                }   
            };
        };

        Err("Seem to have insufficient closing brackets to parse ast!")
    }

    pub fn to_string(self: Ast) -> string::String {
        self.to_string_with_indent(0)
    }

    fn to_string_with_indent(self: Ast, indent: i32) -> string::String {
        let mut output = "".to_string();
        
        match self {
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
                output.pop(); //don't want the ugly extra whitespace
                output.push_str(")");
            },
            Ast::SubArray(v) => {
                output.push_str("\n");
                for _ in 0..indent{
                    output.push_str(" ");
                }
                output.push_str("[");
                for element in v {
                    output.push_str(&element.to_string_with_indent(indent+1)[..]);
                }
                output.pop(); //don't want the ugly extra whitespace
                output.push_str("]");
            },
        };

        output.push_str(" ");
        return output;
    }
}
