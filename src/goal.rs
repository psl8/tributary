use state::State;
use stream::{Stream, StreamElem};
use unify::Unify;

#[derive(Debug, Clone)]
pub enum Op<T: Unify<T>> {
    Unify(T, T),
    Disj(Vec<Goal<T>>),
    Conj(Box<Op<T>>, Box<Op<T>>),
    Delay(Box<Op<T>>),
}

#[derive(Debug, Clone)]
pub struct Goal<T: Unify<T>> {
    // Making state optional could improve eficiency of some
    // operations, such as Conj
    pub state: State<T>,
    pub op: Op<T>,
}

impl<T: Unify<T>> Goal<T> {
    pub fn achieve(mut self) -> Stream<T> {
        let mut stream = Stream::new();
        match self.op {
            Op::Unify(u, v) => stream.add_val(u.unify(v, self.state)),

            Op::Disj(subgoals) => {
                for subgoal in subgoals {
                    stream.add_goal(subgoal);
                }
            }

            Op::Conj(op1, op2) => {
                let work_goal = Goal {
                    state: self.state.clone(),
                    op: *op2,
                };

                let goals = work_goal.achieve();

                for elem in goals.elements {
                    let goal = match elem {
                        StreamElem::Mature(state) => Goal {
                            state,
                            op: *op1.clone(),
                        },
                        StreamElem::Immature(mut new_goal) => {
                            new_goal.op = Op::Conj(op1.clone(), Box::new(new_goal.op));
                            new_goal
                        }
                    };

                    stream.add_goal(goal);
                }
            }

            Op::Delay(next_op) => {
                self.op = *next_op;
                stream.add_goal(self);
            }
        }
        stream
    }
}
