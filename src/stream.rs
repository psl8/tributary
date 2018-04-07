use std::collections::VecDeque;
use unify::{Walk, Unify};
use state::State;
use goal::Goal;

#[derive(Debug)]
pub struct Stream<T: Walk<T>> {
// In order to make this private, we need a way to
// iterate over elements in a stream, so that Conj
// can be implemented. Note that we can't use the
// Iterator below because it matures the stream before
// returning elements
    pub elements: VecDeque<StreamElem<T>>,
}

#[derive(Debug)]
pub enum StreamElem<T: Walk<T>> {
    Mature(State<T>),
    Immature(Goal<T>),
}

impl<T: Walk<T> + Unify<T>> Stream<T> {
    pub fn new() -> Self {
        Stream {
            elements: VecDeque::new(),
        }
    }

    pub fn add_val(&mut self, val: State<T>) {
        self.elements.push_back(StreamElem::Mature(val));
    }

    pub fn add_goal(&mut self, goal: Goal<T>) {
        self.elements.push_back(StreamElem::Immature(goal));
    }

    // Would an in-place merge be better? More efficient?
    pub fn merge(&mut self, mut other: Stream<T>) {
        let mut new_stream = Stream {
            elements: VecDeque::with_capacity(self.elements.len() + other.elements.len()),
        };

        while let Some(val1) = self.elements.pop_front() {
            if let Some(val2) = other.elements.pop_front() {
                // Order here is important for scheduling
                new_stream.elements.push_back(val1);
                new_stream.elements.push_back(val2);
            } else {
                new_stream.elements.push_back(val1);
            }
        }

        while let Some(val) = other.elements.pop_front() {
            new_stream.elements.push_back(val);
        }

        *self = new_stream;
    }

    pub fn mature(&mut self) {
        loop {
            match self.elements.pop_front() {
                Some(StreamElem::Immature(goal)) => {
                    let mut new_stream = goal.achieve();
                    self.merge(new_stream);
                },
                Some(mature) => { self.elements.push_front(mature); break },
                None => break,
            }
        }
    }
}

impl<T: Walk<T> + Unify<T>> Iterator for Stream<T> {
    type Item = State<T>;
    fn next(&mut self) -> Option<Self::Item> {
        let val = self.elements.pop_front();
        match val {
            Some(StreamElem::Mature(val)) => Some(val),
            Some(StreamElem::Immature(thunk)) => {
                self.elements.push_front(StreamElem::Immature(thunk));
                self.mature();
                self.next()
            }
            None => None,
        }
    }
}

