use super::LVal;

pub struct Stream<'a> {
    head: Link<'a>,
}

type Link<'a> = Option<Box<Node<'a>>>;

pub struct Node<'a> {
    elem: StreamElem<'a>,
    next: Link<'a>,
}

pub enum StreamElem<'a> {
    Mature(LVal<'a>),
    Immature(Box<Fn() -> StreamElem<'a>>),
}

impl<'a> Stream<'a> {
    pub fn new() -> Self {
        Stream { head: None }
    }

    pub fn push(&mut self, elem: StreamElem<'a>) {
        let new_node = Box::new(Node {
            elem: elem,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<StreamElem<'a>> {
        self.head.take().map(|boxed_node| {
            let node = *boxed_node;
            self.head = node.next;
            node.elem
        })
    }

    fn mature(&mut self) {
        if let Some(StreamElem::Immature(thunk)) = self.pop() {
            let mut next_elem = thunk();
            while let StreamElem::Immature(thunk) = next_elem {
                next_elem = thunk();
            }
            self.push(next_elem);
        }
    }

	pub fn peek(&self) -> Option<&StreamElem<'a>> {
		self.head.as_ref().map(|node| {
			&node.elem
		})
	}
}

impl<'a> Iterator for Stream<'a> {
	type Item = LVal<'a>;
	fn next(&mut self) -> Option<Self::Item> {
		let val = self.pop();
		match val {
			Some(StreamElem::Mature(val)) => Some(val),
			Some(StreamElem::Immature(thunk)) => {
                self.push(StreamElem::Immature(thunk));
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
        stream.push(StreamElem::Mature(LVal::GInt(3)));
        assert_eq!(LVal::GInt(3), stream.next().unwrap())
    }

    #[test]
    fn stream_immature_iter() {
        let mut stream = Stream::new();
        stream.push(StreamElem::Immature(Box::new(|| { 
            StreamElem::Mature(LVal::GInt(3)) 
        })));

        assert_eq!(LVal::GInt(3), stream.next().unwrap())
    }
}
