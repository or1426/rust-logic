use ast;
use lval;
use lval::lfunc;
use lval::ltype;
use env;

use std::vec;
use std::option;

pub struct LFn{
    tree: ast::Ast,
    environment: env::Env;
    sig: vec::Vec<ltype::LType>,
    ret_type: ltype::LType,
    applied_args: vec::Vec<lval::LVal>
}


impl Clone for LFn{
    fn clone(&self)->LFn{
        LFn{
            tree:self.tree.clone(),
            environment:self.environment.clone(),
            sif:self.sig.clone(),
            ret_type:self.ret_type.clone(),
            applied_args:self.applied_args.clone(),
        }
    }
}
