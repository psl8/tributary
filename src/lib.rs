use std::collections::HashMap;
use std::cell::Cell;
use std::fmt::Debug;
use std::any::Any;

trait Walk : Debug {
    fn walk(&self, &SMap) -> Box<Walk>;
    fn as_any(&self) -> &Any;
}

struct SMap(HashMap<LVar, Box<Walk>>);

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

impl Walk for LVar {
    fn walk(&self, map: &SMap) -> Box<Walk> {
        if let Some(var) = map.0.get(self) {
            var.walk(map)
        } else {
            Box::new(*self)
        }
    }
    fn as_any(&self) -> &Any { self }
}

macro_rules! opaque_impl_walk {
    ($type:ty) =>
        (
            impl Walk for $type {
                fn walk(&self, _: &SMap) -> Box<Walk> {
                    Box::new(*self)
                }
                fn as_any(&self) -> &Any { self }
            }
        );
    ($type:ty, $($types:ty),+) =>
        (
            opaque_impl_walk!($type);
            opaque_impl_walk!($($types),+);
        )
}

opaque_impl_walk!(u8, u16, u32, u64, i8, i16, i32, i64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut s_map = SMap(HashMap::new());
        let l1 = LVar::new();
        let l2 = LVar::new();
        s_map.0.insert(l1, Box::new(l2));
        s_map.0.insert(l2, Box::new(0_u32));
        assert_eq!(Some(&0), l1.walk(&s_map).as_any().downcast_ref::<u32>());
    }
}
