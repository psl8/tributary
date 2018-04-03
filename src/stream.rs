use super::LVal;
use std::collections::VecDeque;

pub struct Stream<'a> {
    elements: VecDeque<StreamElem<'a>>,
}

pub enum StreamElem<'a> {
    Mature(LVal<'a>),
    Immature(Box<Fn() -> StreamElem<'a>>),
}

impl<'a> Stream<'a> {
    pub fn new() -> Self {
        Stream { elements: VecDeque::new() }
    }

    pub fn add_val(&mut self, val: LVal<'a>) {
        self.elements.push_back(StreamElem::Mature(val));
    }

    pub fn add_thunk(&mut self, thunk: Box<Fn() -> StreamElem<'a>>) {
        self.elements.push_back(StreamElem::Immature(thunk));
    }

    pub fn merge(mut self, mut other: Stream<'a>) -> Stream<'a> {
        let mut new_stream = Stream {
            elements: VecDeque::with_capacity(self.elements.len() + other.elements.len())
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

    // TODO: make this take into account that we might have multiple merged
    // streams with the one at the front diverging. As of now this simply hangs
    // on a divergent stream.
    pub fn mature(&mut self) {
        if let Some(StreamElem::Immature(thunk)) = self.elements.pop_front() {
            let mut next_elem = thunk();
            while let StreamElem::Immature(thunk) = next_elem {
                next_elem = thunk();
            }
            self.elements.push_front(next_elem);
        }
    }
}

impl<'a> Iterator for Stream<'a> {
	type Item = LVal<'a>;
	fn next(&mut self) -> Option<Self::Item> {
		let val = self.elements.pop_front();
		match val {
			Some(StreamElem::Mature(val)) => Some(val),
			Some(StreamElem::Immature(thunk)) => {
                self.elements.push_front(StreamElem::Immature(thunk));
				self.mature();
				self.next()
			},
			None => None,
		}
	}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stream_mature_iter() {
        let mut stream = Stream::new();

        stream.add_val(LVal::GInt(3));

        assert_eq!(LVal::GInt(3), stream.next().unwrap())
    }

    #[test]
    fn stream_immature_iter() {
        let mut stream = Stream::new();

        stream.add_thunk(Box::new(|| {
            StreamElem::Mature(LVal::GInt(3)) 
        }));

        assert_eq!(LVal::GInt(3), stream.next().unwrap())
    }

    #[test]
    fn stream_mixed_iter() {
        let mut stream = Stream::new();

        stream.add_val(LVal::GInt(3));
        stream.add_thunk(Box::new(|| {
            StreamElem::Mature(LVal::GStr("Hello")) 
        }));

        assert_eq!(LVal::GInt(3), stream.next().unwrap());
        assert_eq!(LVal::GStr("Hello"), stream.next().unwrap());
    }
}
