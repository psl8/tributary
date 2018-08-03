#![feature(trace_macros)]
#[macro_use]
extern crate tributary;

use tributary::prelude::*;

fn main() {
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
                Unify(Var(u), List(vec![Sym("foo"), Sym("bar"), Sym("baz")])),
                Unify(Var(q), List(vec![Sym("foo"), Var(r)])),
                Unify(Var(u), Var(q)),
            ],

            /*
            [
                Unify(Var(q), Var(r)),
                Unify(Var(r), Var(q)),
            ]
            */
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
                Unify(Var(u), List(vec![Sym("foo"), Sym("bar"), Sym("baz")])),
                Unify(Var(q), List(vec![Sym("foo"), Var(r)])),
                Unify(Var(u), Var(q)),
            ],

            /*
            [
                Unify(Var(q), Var(r)),
                Unify(Var(r), Var(q)),
            ]
            */
        })
    };

    for answer in stream {
        print!("[ ");
        for value in answer {
            print!("{} ", value);
        }
        println!("]");
    }
}
