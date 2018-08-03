use state::State;
use stream::{Stream, StreamElem};
use unify::Unify;

#[derive(Debug, Clone)]
pub enum Op<T: Unify<T>> {
    Unify(T, T),
    Disj(Vec<Op<T>>),
    // FIXME: Conj should be a VecDeque?
    // Does it matter?
    Conj(Vec<Op<T>>),
    Delay(Box<Op<T>>),
}

#[derive(Debug, Clone)]
pub struct Goal<T: Unify<T>> {
    pub state: State<T>,
    pub op: Op<T>,
}

impl<T: Unify<T>> Goal<T> {
    pub fn achieve(mut self) -> Stream<T> {
        let mut stream = Stream::new();
        if self.state.s_map.is_err() {
            return stream;
        }

        match self.op {
            Op::Unify(u, v) => stream.add_val(u.unify(v, self.state)),

            Op::Disj(ops) => {
                for op in ops {
                    let subgoal = Goal {
                        state: self.state.clone(),
                        op,
                    };
                    stream.add_goal(subgoal);
                }
            }

            Op::Conj(mut ops) => {
                if let Some(head_op) = ops.pop() {
                    let work_goal = Goal {
                        state: self.state,
                        op: head_op,
                    };

                    let goals = work_goal.achieve();

                    for elem in goals.elements {
                        let goal = match elem {
                            StreamElem::Mature(state) => Goal {
                                state,
                                op: Op::Conj(ops.clone()),
                            },
                            StreamElem::Immature(mut new_goal) => {
                                let mut new_goals = ops.clone();
                                new_goals.push(new_goal.op);
                                new_goal.op = Op::Conj(new_goals);
                                new_goal
                            }
                        };

                        stream.add_goal(goal);
                    }
                } else {
                    stream.add_val(self.state);
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
