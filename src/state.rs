use std::collections::HashMap;
use std::fmt::{self, Display};
use unify::Unify;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct LVar {
    id: u64,
}

impl Display for LVar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "var{}", self.id)
    }
}

#[derive(Clone, Debug)]
pub struct State<T: Unify<T>> {
    s_map: Result<HashMap<LVar, T>, ()>,
    next_id: u64,
}

impl<T: Unify<T>> Default for State<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Unify<T>> Display for State<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.s_map {
            Ok(ref map) => {
                let mut output = "{ ".to_owned();
                for (key, value) in map.iter() {
                    output.push_str(&format!("{}: {}, ", key, value));
                }
                output.push('}');
                write!(f, "{}", output)
            }
            Err(()) => write!(f, "nil"),
        }
    }
}

impl<T: Unify<T>> State<T> {
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
}
