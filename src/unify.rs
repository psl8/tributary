use state::{LVar, State};
use std::fmt::{self, Debug, Display};

pub trait Unify<T: Unify<T>>: Debug + Display + Clone {
    fn unify(&self, other: T, state: State<T>) -> State<T>;
    fn walk(&self, state: &State<T>) -> T;
}

#[derive(PartialEq, Clone, Debug)]
pub enum LVal<'a> {
    Var(LVar),
    Int(i64),
    Float(f64),
    Str(String),
    Sym(&'a str),
    List(Vec<LVal<'a>>),
}

impl<'a> Display for LVal<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LVal::Var(var) => write!(f, "{}", var),
            LVal::Int(i) => write!(f, "{}", i),
            LVal::Float(i) => write!(f, "{}", i),
            LVal::Str(ref s) => write!(f, "\"{}\"", s),
            LVal::Sym(s) => write!(f, "{}", s),
            LVal::List(ref list) => {
                let mut output = "[".to_owned();
                for elem in list {
                    output.push_str(&format!("{}, ", elem));
                }
                if !list.is_empty() {
                    output.pop();
                    output.pop();
                }
                output.push(']');
                write!(f, "{}", output)
            }
        }
    }
}

impl<'a> Unify<LVal<'a>> for LVal<'a> {
    fn unify(&self, other: LVal<'a>, mut state: State<LVal<'a>>) -> State<LVal<'a>> {
        let u = other.walk(&state);
        let v = self.walk(&state);

        // TODO: should variables be able to match multiple list elements?
        if let LVal::List(list1) = u {
            match v {
                LVal::List(list2) => for (elem1, elem2) in list1.iter().zip(list2) {
                    state = elem1.unify(elem2, state);
                },
                LVal::Var(var) => state.add(var, LVal::List(list1)),
                _ => state.fail(),
            }
        } else if let LVal::Var(var) = u {
            state.add(var, v);
        } else if let LVal::Var(var) = v {
            state.add(var, u);
        // What should NaN do here?
        // Does NaN unify with NaN?
        } else if u != v {
            state.fail();
        }

        state
    }

    fn walk(&self, state: &State<LVal<'a>>) -> LVal<'a> {
        // This error handling is less than ideal
        if state.has_failed() {
            return LVal::Sym("nil");
        }

        match *self {
            LVal::Var(var) => match state.get(var) {
                // Check if self == val?
                // Would this break *all* cycles?
                Some(val) => val.walk(state),
                None => LVal::Var(var),
            },
            LVal::Int(i) => LVal::Int(i),
            LVal::Float(i) => LVal::Float(i),
            LVal::Str(ref s) => LVal::Str(s.clone()),
            LVal::Sym(s) => LVal::Sym(s),
            LVal::List(ref list) => LVal::List(list.iter().map(|elem| elem.walk(&state)).collect()),
        }
    }
}
