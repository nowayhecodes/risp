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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Lval::Risp(_cells) => write!(f, "<toplevel>"),
            Lval::Fun(lf) => match lf {
                LvalFn::Builtin(name, _) => write!(f, "<builtin: {}>", name),
                LvalFn::Lambda(_, formals, body) => write!(f, "(\\ {} {})", formals, body),
            },
            Lval::Num(n) => write!(f, "{}", n),
            Lval::Sym(s) => write!(f, "{}", s),
            Lval::Sexpr(cell) => write!(f, "({})", lval_expr_print(cell)),
            Lval::Qexpr(cell) => write!(f, "{{{}}}", lval_expr_print(cell)),
        }
    }
}

fn lval_expr_print(cell: &[Box<Lval>]) -> String {
    let mut ret = String::new();
    for i in 0..cell.len() {
        ret.push_str(&format!("{}", cell[i]));
        if i < cell.len() - 1 {
            ret.push_str(" ");
        }
    }
    ret
}

pub fn lval_num(number: i64) -> Box<Lval> {
    Box::new(Lval::Num(number))
}

pub fn lval_add(v: &mut Lval, x: &Lval) -> Result<()> {
    match *v {
        Lval::Sexpr(ref mut children)
        | Lval::Qexpr(ref mut children)
        | Lval::Risp(ref mut children) => {
            children.push(Box::new(x.clone()));
        }
        _ => return Err(RispError::NoChildren),
    }
    Ok(())
}

pub fn lval_pop(v: &mut Lval, i: usize) -> RispResult {
    match *v {
        Lval::Sexpr(ref mut children)
        | Lval::Qexpr(ref mut children)
        | Lval::Risp(ref mut children) => {
            let ret = (&children[i]).clone();
            children.remove(i);
            Ok(ret)
        }
        _ => Err(RispError::NoChildren),
    }
}
