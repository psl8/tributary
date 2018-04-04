use state::{LVar, State};

pub trait Walk<T: Walk<T>> {
    fn walk(&self, &State<T>) -> T;
}

pub trait Unify<T: Walk<T>> {
    fn unify(&self, T, State<T>) -> State<T>;
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum LVal<'a> {
    Var(LVar),
    Int(u64),
    Str(&'a str),
    Nil,
}

impl<'a> Walk<LVal<'a>> for LVal<'a> {
    fn walk(&self, state: &State<LVal<'a>>) -> LVal<'a> {
        if state.has_failed() {
            return LVal::Nil;
        }

        match *self {
            LVal::Var(lvar) => if let Some(var) = state.get(lvar) {
                var.walk(state)
            } else {
                LVal::Var(lvar)
            },
            LVal::Int(i) => LVal::Int(i),
            LVal::Str(s) => LVal::Str(s),
            LVal::Nil => LVal::Nil,
        }
    }
}

impl<'a> Unify<LVal<'a>> for LVal<'a> {
    fn unify(&self, other: LVal<'a>, mut state: State<LVal<'a>>) -> State<LVal<'a>> {
        let u = other.walk(&state);
        let v = self.walk(&state);
        if u == v {
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
