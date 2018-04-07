use goal::*;
use state::*;
use unify::LVal::*;
use unify::*;

#[test]
fn simple_unify() {
    let mut state = State::new();
    let l1 = state.make_var();
    let l2 = state.make_var();
    state.add(l1, Var(l2));
    state.add(l2, Str("Hello"));

    let state = Var(l1).unify(Var(l2), state);
    assert_eq!(Str("Hello"), Var(l1).walk(&state));
}

#[test]
fn simple_stream() {
    let mut state = State::new();
    let l1 = state.make_var();
    let l2 = state.make_var();
    state.add(l1, Var(l2));
    state.add(l2, Str("Hello"));

    let mut stream = state.unify(Var(l1), Var(l2));
    assert_eq!(Str("Hello"), Var(l1).walk(&stream.next().unwrap()));
}

#[test]
fn simple_goal() {
    let mut state = State::new();
    let l1 = state.make_var();
    let l2 = state.make_var();
    state.add(l1, Var(l2));
    state.add(l2, Str("Hello"));

    let goal = Goal {
        state,
        op: Op::Unify(Var(l1), Var(l2)),
    };
    let mut stream = goal.achieve();

    assert_eq!(Str("Hello"), Var(l1).walk(&stream.next().unwrap()));
}

#[test]
fn conj_test() {
    let mut state = State::new();
    let l1 = state.make_var();
    let l2 = state.make_var();

    let goal = Goal {
        state,
        op: Op::Conj(
            Box::new(Op::Unify(Var(l1), Sym("true"))),
            Box::new(Op::Unify(Var(l1), Var(l2))),
            ),
    };
    let mut stream = goal.achieve();

    let answer = stream.next().unwrap();
    
    assert_eq!(Var(l1).walk(&answer), Var(l2).walk(&answer));
    assert_eq!(Sym("true"), Var(l2).walk(&answer));
}
