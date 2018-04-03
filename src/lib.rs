use std::collections::HashMap;
use std::cell::Cell;

pub mod stream;

trait Walk<T: Walk<T>> {
    type Output;
    fn walk(&self, &SMap<T>) -> Self::Output;
}

trait Unify<T: Walk<T>> {
    fn unify(&self, T, SMap<T>) -> Option<SMap<T>>;
}

struct SMap<T: Walk<T>>(HashMap<LVar, T>);

impl<'a> SMap<LVal<'a>> {
    fn add(&mut self, var: LVal, val: LVal<'a>) {
        if let LVal::LVar(lvar) = var {
            self.0.insert(lvar, val);
        } else {
            panic!("Key of an SMap needs to be an LVar");
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct LVar {
    id: u64,
}

impl LVar {
    const CURRENT_ID: Cell<u64>  = Cell::new(0);
    pub fn new() -> LVar {
        let id = Self::CURRENT_ID.get() + 1;
        Self::CURRENT_ID.set(id);
        LVar { id }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum LVal<'a> {
    LVar(LVar),
    GInt(u64),
    GStr(&'a str),
    Nil,
}

impl<'a> LVal<'a> {
    fn is_lvar(&self) -> bool {
        if let LVal::LVar(_) = *self {
            true
        } else {
            false
        }
    }
}

impl<'a> Walk<LVal<'a>> for LVal<'a> {
    type Output = LVal<'a>;
    fn walk(&self, map: &SMap<LVal<'a>>) -> LVal<'a> {
        match *self {
            LVal::LVar(lvar) => 
                if let Some(var) = map.0.get(&lvar) {
                    var.walk(map)
                } else {
                    LVal::LVar(lvar)
                }
            LVal::GInt(i) => LVal::GInt(i),
            LVal::GStr(s) => LVal::GStr(s),
            LVal::Nil => LVal::Nil
        }
    }
}

impl<'a> Unify<LVal<'a>> for LVal<'a> {
    fn unify(&self, other: LVal<'a>, mut map: SMap<LVal<'a>>) -> Option<SMap<LVal<'a>>> {
        let u = other.walk(&map);
        let v = self.walk(&map);
        if u == v {
            Some(map)
        } else if u.is_lvar() {
            map.add(u, v);
            Some(map)
        } else if v.is_lvar() {
            map.add(v, u);
            Some(map)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut s_map = SMap(HashMap::new());
        let l1 = LVar::new();
        let l2 = LVar::new();
        s_map.0.insert(l1, LVal::LVar(l2));
        s_map.0.insert(l2, LVal::GStr("Hello"));
        assert_eq!(LVal::GStr("Hello"), LVal::LVar(l1).walk(&s_map))
    }
}
