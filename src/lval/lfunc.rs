use lval;

pub struct BuiltinFn{
    f: fn(lval::LVal) -> lval::LVal,
}

pub enum LFunc{
    Builtin(BuiltinFn),
}
