use std::collections::HashMap;
use unify::{Walk, Unify};
use stream::Stream;

#[derive(Clone, Debug)]
pub struct State<T: Walk<T>> {
    s_map: Result<HashMap<LVar, T>, ()>,
    next_id: u64,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct LVar {
    id: u64,
}

impl<T: Unify<T> + Walk<T>> State<T> {
    pub fn new() -> Self {
        State {
            s_map: Ok(HashMap::new()),
            next_id: 0,
        }
    }

    pub fn fail(&mut self) {
        self.s_map = Err(());
    }

    pub fn has_failed(&self) -> bool {
        match self.s_map {
            Err(()) => true,
            Ok(_) => false,
        }
    }

    pub fn make_var(&mut self) -> LVar {
        let var = LVar { id: self.next_id };
        self.next_id += 1;
        var
    }

    pub fn add(&mut self, var: LVar, val: T) {
        if let Ok(ref mut map) = self.s_map {
            map.insert(var, val);
        }
    }

    pub fn get(&self, var: LVar) -> Option<&T> {
        match self.s_map {
            Ok(ref map) => map.get(&var),
            Err(()) => None,
        }
    }

    pub fn and_then<F>(self, f: F) -> Self
    where
        F: FnOnce(Self) -> Self,
    {
        f(self)
    }

    // TODO: This should probably be on the Stream struct actually
    pub fn unify(self, u: T, v: T) -> Stream<T> {
        let state = u.unify(v, self);
        let mut stream = Stream::new();
        stream.add_val(state);
        stream
    }
}
