type LvalChildren = Vec<Box<Lval>>;
pub type LBuiltin = fn(&mut Lval) -> RispResult;

#[derive(Clone)]
pub enum LvalFn {
    Builtin(String, LBuiltin),
    Lambda(HashMap<String, Box<Lval>>, Box<Lval>, Box<Lval>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Lval {
    Fun(LvalFn),
    Num(i64),
    Sym(String),
    Sexpr(LvalChildren),
    Qexpr(LvalChildren),
}

impl fmt::Display for Lval {
    fn fmt()
}