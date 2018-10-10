use std::iter::Iterator;

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

struct Node<T> {
    data: T,
    next_node: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(t: T) -> Self {
        Node {
            data: t,
            next_node: None,
        }
    }
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { head: None }
    }

    pub fn len(&self) -> usize {
        let mut i: usize = 0;
        let mut n = &self.head;
        loop {
            match n {
                Some(boxednode) => {
                    i += 1;
                    n = &boxednode.next_node;
                }
                None => break,
            }
        }
        i
    }

    pub fn push(&mut self, _element: T) {
        let mut b = Box::new(Node::new(_element));
        b.next_node = self.head.take();
        self.head = Some(b);
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            Some(mut bn) => {
                self.head = bn.next_node.take();
                Some(bn.data)
            }
            None => None,
        }
    }

    pub fn peek<'a>(&'a self) -> Option<&'a T> {
        match &self.head {
            Some(b) => Some(&b.data),
            None => None,
        }
    }
}

impl<T: Clone> SimpleLinkedList<T> {
    pub fn rev(&self) -> SimpleLinkedList<T> {
        let mut out = SimpleLinkedList::new();
        let mut n = &self.head;
        loop {
            match n {
                Some(boxednode) => {
                    out.push(boxednode.data.clone());
                    n = &boxednode.next_node;
                }
                None => break,
            }
        }
        out
    }
}

impl<'a, T: Clone> From<&'a [T]> for SimpleLinkedList<T> {
    fn from(_item: &[T]) -> Self {
        let mut out = SimpleLinkedList::new();
        for i in _item {
            out.push(i.clone());
        }
        out
    }
}

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(mut self) -> Vec<T> {
        let mut out = Vec::new();
        loop {
            match self.pop() {
                Some(b) => {
                    out.insert(0, b);
                }
                None => break,
            }
        }
        out
    }
}

pub struct IntoIter<T> {
    list: SimpleLinkedList<T>,
}

impl<T> IntoIter<T> {
    pub fn new(l: SimpleLinkedList<T>) -> Self {
        IntoIter { list: l }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.list.pop()
    }
}

impl<T> IntoIterator for SimpleLinkedList<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self)
    }
}
