use std::mem;

pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });

        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        //let result;
        match self.pop_node() {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
        //result
        //unimplemented!()
    }

    fn pop_node(&mut self) -> Link {
        mem::replace(&mut self.head, Link::Empty)
    }
}

impl Drop for List {
    fn drop(&mut self) {
        //self.head.drop();

        while let Link::More(boxed_node) = self.pop_node() {
            self.head = boxed_node.next;
            self.pop_node();
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn basics() {
        
        //Check empty list behaves right
        let mut list = List::new();

        //Populate the list
        list.push(1);
        list.push(2);
        list.push(3);

        //Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        //Populate again
        list.push(4);
        list.push(5);

        //Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        //Check Exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}

