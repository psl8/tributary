#[macro_export]
macro_rules! conde {
    ($e:tt $(,)*) => {
        $crate::goal::Op::Disj(vec![Conj(vec!$e)])
    };
    ($head:tt, $($tail:tt),* $(,)*) => ({
        let mut v = vec![Conj(vec!$head)];
        conde!(@conj v $($tail),*);
        $crate::goal::Op::Disj(v)
    });
    (@conj $v:ident $head:tt, $($tail:tt),*) => {
        $v.push(Conj(vec!$head));
        conde!(@conj $v $($tail),*);
    };
    (@conj $v:ident $head:tt) => {
        $v.push(Conj(vec!$head));
    };
}

#[macro_export]
macro_rules! fresh {
    (@vars $state:ident, $name:ident) => {
        let $name = $state.make_var();
    };

    (@vars $state:ident, $name:ident $($names:ident)*) => {
        let $name = $state.make_var();
        fresh!(@vars $state, $($names)*);
    };

    ([ $($names:ident)* ] $body:tt) => ({
        let mut state = $crate::state::State::new();
        fresh!(@vars state, $($names)*);
        Goal {
            state,
            op: $crate::goal::Op::Conj(vec!$body),
        }
    })
}

#[macro_export]
macro_rules! run {
    (take: *; [ $($names:ident)* ] $body:tt) => {
        fresh!([$($names)*] $body).achieve().map(|s| s.reify())
    };

    (take: $num:expr; [ $($names:ident)* ] $body:tt) => {
        fresh!([$($names)*] $body).achieve().take($num).map(|s| s.reify())
    }
}
