use std::fmt::{self, Display};
use unify::Unify;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct LVar {
    id: usize,
}

impl Display for LVar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "_{}", self.id)
    }
}

#[derive(Clone, Debug)]
pub struct State<T: Unify<T>> {
    pub s_map: Result<Vec<Option<T>>, ()>,
    next_id: usize,
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
                for (i, value) in map.iter().enumerate() {
                    match value {
                        Some(value) => output.push_str(&format!("_{}: {}, ", i, value)),
                        None => output.push_str(&format!("_{}: _{}, ", i, i)),
                    }
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
            s_map: Ok(Vec::new()),
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
        match self.s_map {
            Ok(ref mut map) => map.push(None),
            Err(()) => (),
        }
        var
    }

    pub fn add(&mut self, var: LVar, val: T) {
        match self.s_map {
            Ok(ref mut map) => map[var.id] = Some(val),
            Err(()) => (),
        }
    }

    pub fn get(&self, var: LVar) -> &Option<T> {
        match self.s_map {
            Ok(ref map) => map.get(var.id).unwrap_or(&None),
            Err(()) => &None,
        }
    }

    fn reify_vars(self, vars: &[LVar]) -> Vec<T> {
        let mut reified_vars = Vec::new();

        for var in vars {
            if let Some(val) = self.get(*var) {
                reified_vars.push(val.walk(&self))
            }
        }

        reified_vars
    }

    pub fn reify(self) -> Vec<T> {
        let mut vars = Vec::new();
        for i in 0..self.next_id {
            vars.push(LVar { id: i });
        }
        self.reify_vars(&vars)
    }
}
