use unify::*;
use state::*;
//use stream::*;

#[test]
fn simple_unify() {
    let mut state = State::new();
    let l1 = state.make_var();
    let l2 = state.make_var();
    state.add(l1, LVal::Var(l2));
    state.add(l2, LVal::Str("Hello"));
    let state = LVal::Var(l1).unify(LVal::Var(l2), state);
    assert_eq!(LVal::Str("Hello"), LVal::Var(l1).walk(&state));
}

#[test]
fn simple_stream() {
    let mut state = State::new();
    let l1 = state.make_var();
    let l2 = state.make_var();
    state.add(l1, LVal::Var(l2));
    state.add(l2, LVal::Str("Hello"));
    let mut stream = state.unify(LVal::Var(l1), LVal::Var(l2));
    assert_eq!(LVal::Str("Hello"), LVal::Var(l1).walk(&stream.next().unwrap()));
}


/*
#[test]
fn stream_mature() {
    let mut stream = Stream::new();

    stream.add_val(LVal::Int(3));

    assert_eq!(LVal::Int(3), stream.next().unwrap())
}

#[test]
fn stream_immature() {
    let mut stream = Stream::new();

    stream.add_thunk(Box::new(|| StreamElem::Mature(LVal::Int(3))));

    assert_eq!(LVal::Int(3), stream.next().unwrap())
}

#[test]
fn stream_mixed() {
    let mut stream = Stream::new();

    stream.add_val(LVal::Int(3));
    stream.add_thunk(Box::new(|| StreamElem::Mature(LVal::Str("Hello"))));

    assert_eq!(LVal::Int(3), stream.next().unwrap());
    assert_eq!(LVal::Str("Hello"), stream.next().unwrap());
}

#[test]
fn stream_mixed_nested_thunks() {
    let mut stream = Stream::new();

    stream.add_thunk(Box::new(|| {
        StreamElem::Immature(Box::new(|| StreamElem::Mature(LVal::Str("Hello"))))
    }));
    stream.add_val(LVal::Int(3));

    assert_eq!(LVal::Int(3), stream.next().unwrap());
    assert_eq!(LVal::Str("Hello"), stream.next().unwrap());
}
*/
