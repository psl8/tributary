use std::collections::VecDeque;
use unify::Walk;
use state::State;

pub struct Stream<T: Walk<T>> {
    elements: VecDeque<StreamElem<T>>,
}

pub enum StreamElem<T: Walk<T>> {
    Mature(State<T>),
    Immature(Box<Fn() -> StreamElem<T>>),
}

impl<T: Walk<T>> Stream<T> {
    pub fn new() -> Self {
        Stream {
            elements: VecDeque::new(),
        }
    }

    pub fn add_val(&mut self, val: State<T>) {
        self.elements.push_back(StreamElem::Mature(val));
    }

    pub fn add_thunk(&mut self, thunk: Box<Fn() -> StreamElem<T>>) {
        self.elements.push_back(StreamElem::Immature(thunk));
    }

    pub fn merge(mut self, mut other: Stream<T>) -> Stream<T> {
        let mut new_stream = Stream {
            elements: VecDeque::with_capacity(self.elements.len() + other.elements.len()),
        };

        while let Some(val1) = self.elements.pop_front() {
            if let Some(val2) = other.elements.pop_front() {
                new_stream.elements.push_back(val1);
                new_stream.elements.push_back(val2);
            } else {
                new_stream.elements.push_back(val1);
            }
        }

        while let Some(val) = other.elements.pop_front() {
            new_stream.elements.push_back(val);
        }

        new_stream
    }

    pub fn mature(&mut self) {
        if let Some(StreamElem::Immature(thunk)) = self.elements.pop_front() {
            let mut next_elem = thunk();
            while let StreamElem::Immature(thunk) = next_elem {
                next_elem = thunk();
            }
            self.elements.push_back(next_elem);
        }
    }
}

impl<T: Walk<T>> Iterator for Stream<T> {
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

