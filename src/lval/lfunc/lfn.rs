use ast;
use lval;
use lval::lfunc;
use lval::ltype;
use env;

use std::vec;
use std::option;

pub struct LFn {
    tree: ast::Ast,
    environment: env::Env;
    sig: vec::Vec<ltype::LType>,
    ret_type: ltype::LType,
    applied_args: vec::Vec<lval::LVal>
}
