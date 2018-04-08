extern crate tributary;

use tributary::*;

fn main() {
    let mut state = State::new();
    let l1 = state.make_var();
    let l2 = state.make_var();
    let l3 = state.make_var();
    //let l4 = state.make_var();

    //state.add(l1, Var(l4));
    //state.add(l2, Str("Hello"));
    state.add(l3, Nil);

    println!("before: {}", state);

    let goal = Goal {
        state,
        op: Op::Conj(
            Box::new(Op::Unify(Var(l1), Sym("true"))),
            Box::new(Op::Unify(Var(l1), Var(l2))),
        ),
    };
    let mut stream = Stream::new();
    stream.add_goal(goal);

    for elem in stream {
        println!("after: {}", elem);
        println!("l1: {}, l2: {}", Var(l1).walk(&elem), Var(l2).walk(&elem));
        //println!("{:?}", elem);
    }
}
