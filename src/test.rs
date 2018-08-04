use prelude::*;

#[test]
fn simple_unify() {
    let mut state = State::new();
    let l1 = state.make_var();
    let l2 = state.make_var();
    state.add(l1, Var(l2));
    state.add(l2, Sym("Hello"));

    let state = Var(l1).unify(Var(l2), state);
    assert_eq!(Sym("Hello"), Var(l1).walk(&state));
}

#[test]
fn simple_goal() {
    let mut state = State::new();
    let l1 = state.make_var();
    let l2 = state.make_var();
    state.add(l1, Var(l2));
    state.add(l2, Sym("Hello"));

    let goal = Goal {
        state,
        op: Unify(Var(l1), Var(l2)),
    };
    let mut stream = goal.achieve();

    assert_eq!(Sym("Hello"), Var(l1).walk(&stream.next().unwrap()));
}

#[test]
fn conj_test() {
    let mut state = State::new();
    let l1 = state.make_var();
    let l2 = state.make_var();

    let goal = Goal {
        state,
        op: Conj(vec![Unify(Var(l1), Sym("true")), Unify(Var(l1), Var(l2))]),
    };
    let mut stream = goal.achieve();

    let answer = stream.next().unwrap();

    assert_eq!(Var(l1).walk(&answer), Var(l2).walk(&answer));
    assert_eq!(Sym("true"), Var(l2).walk(&answer));
}

#[test]
fn list_test() {
    let mut state = State::new();
    let l1 = state.make_var();
    let l2 = state.make_var();
    let l3 = state.make_var();

    let goal = Goal {
        state,
        op: Conj(vec![
            Unify(
                Var(l1),
                List(vec![Sym("foo"), List(vec![Sym("bar"), Sym("baz")])]),
            ),
            Unify(Var(l2), List(vec![Sym("foo"), Var(l3)])),
            Unify(Var(l1), Var(l2)),
        ]),
    };
    let mut stream = goal.achieve();

    let answer = stream.next().unwrap();

    assert_eq!(Var(l1).walk(&answer), Var(l2).walk(&answer));
    assert_eq!(
        List(vec![Sym("foo"), List(vec![Sym("bar"), Sym("baz")])]),
        Var(l2).walk(&answer)
    );
    assert_eq!(List(vec![Sym("bar"), Sym("baz")]), Var(l3).walk(&answer));
}

use rust_test::Bencher;
#[bench]
fn conj_bench(b: &mut Bencher) {
    b.iter(|| {
        let stream = run! {
            take: 1;
            [q r u]
            (conde! {
                [
                    Unify(Var(u), Sym("nil")),
                    Unify(Var(q), Sym("true")),
                    Unify(Var(r), Var(q)),
                ],

                [
                    Unify(Var(u), Sym("nil")),
                    Unify(Var(q), Sym("false")),
                    Unify(Var(r), Var(q)),
                ],
            })
        };
        stream.collect::<Vec<_>>()
    });
}

#[bench]
fn longer_bench(b: &mut Bencher) {
    b.iter(|| {
        let stream = run! {
            take: *;
            [q r u]
            (conde! {
                [
                    Unify(Var(u), Sym("nil")),
                    Unify(Var(q), Sym("true")),
                    Unify(Var(r), Var(q)),
                ],

                [
                    Unify(Var(u), Sym("nil")),
                    Unify(Var(q), Sym("false")),
                    Unify(Var(r), Var(q)),
                ],

                [
                    Unify(Var(u), List(vec![Sym("foo"), List(vec![Sym("bar"), Sym("baz")])])),
                    Unify(Var(q), List(vec![Sym("foo"), Var(r)])),
                    Unify(Var(u), Var(q)),
                ],

            },

            conde! {
                [
                    Unify(Var(u), Sym("nil")),
                    Unify(Var(q), Sym("true")),
                    Unify(Var(r), Var(q)),
                ],

                [
                    Unify(Var(u), Sym("nil")),
                    Unify(Var(q), Sym("false")),
                    Unify(Var(r), Var(q)),
                ],

                [
                    Unify(Var(u), List(vec![Sym("foo"), List(vec![Sym("bar"), Sym("baz")])])),
                    Unify(Var(q), List(vec![Sym("foo"), Var(r)])),
                    Unify(Var(u), Var(q)),
                ],

            })
        };
        stream.collect::<Vec<_>>()
    });
}
