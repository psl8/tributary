use state::{LVar, State};
use std::fmt::{self, Display, Debug};

// TODO: Remove Walk trait
// Having different traits for Walk and Unify seems
// redundant. Almost every interface needs both. The
// `walk` method can be added to Unify
pub trait Walk<T: Walk<T>> : Debug + Display + Clone {
    fn walk(&self, state: &State<T>) -> T;
}

pub trait Unify<T: Walk<T>> {
    fn unify(&self, other: T, state: State<T>) -> State<T>;
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum LVal<'a> {
    Var(LVar),
    Int(i64),
    Str(&'a str),
    Sym(&'a str),
    // A vec might be better
    Pair(Box<LVal<'a>>, Box<LVal<'a>>),
    Nil,
}

impl<'a> Display for LVal<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LVal::Var(var) => write!(f, "{}", var),
            LVal::Int(i) => write!(f, "{}", i),
            LVal::Str(s) => write!(f, "\"{}\"", s),
            LVal::Sym(s) => write!(f, "{}", s),
            LVal::Pair(car, cdr) => write!(f, "({}, {})", *car, *cdr),
            LVal::Nil => write!(f, "nil"),
        }
    }
}


impl<'a> Walk<LVal<'a>> for LVal<'a> {
    fn walk(&self, state: &State<LVal<'a>>) -> LVal<'a> {
        if state.has_failed() {
            return LVal::Nil;
        }

        match *self {
            LVal::Var(lvar) => match state.get(lvar) {
                Some(var) => var.walk(state),
                None => LVal::Var(lvar),
            },
            LVal::Int(i) => LVal::Int(i),
            LVal::Str(s) => LVal::Str(s),
            LVal::Sym(s) => LVal::Sym(s),
            LVal::Pair(ref car, ref cdr) => {
                LVal::Pair(
                    Box::new(car.walk(state)),
                    Box::new(cdr.walk(state)),
                    )
            }
            LVal::Nil => LVal::Nil,
        }
    }
}

impl<'a> Unify<LVal<'a>> for LVal<'a> {
    fn unify(&self, other: LVal<'a>, mut state: State<LVal<'a>>) -> State<LVal<'a>> {
        let u = other.walk(&state);
        let v = self.walk(&state);

        if u == v {
            // no-op
        } else if let LVal::Var(var) = u {
            state.add(var, v);
        } else if let LVal::Var(var) = v {
            state.add(var, u);
        } else {
            state.fail();
        }
        state
    }
}
