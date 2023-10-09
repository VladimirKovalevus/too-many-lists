use std::mem::replace;
pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

pub trait Drop {
    fn drop(&mut self);
}
pub struct Node {
    val: i32,
    next: Link,
}

impl List {
    fn new() -> List {
        List { head: Link::Empty }
    }
    fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            val: elem,
            next: replace(&mut self.head, Link::Empty),
        });
        self.head = Link::More(new_node);
    }
    fn pop(&mut self) -> Option<i32> {
        match replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.val)
            }
        }
    }
}
impl Drop for List {
    fn drop(&mut self) {
        while let Some(_) = self.pop() {}
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
