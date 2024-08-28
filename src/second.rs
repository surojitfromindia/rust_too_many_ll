struct Node {
    elem: i32,
    next: Link,
}
type Link = Option<Box<Node>>;

pub struct List {
    head: Link,
}
impl List {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: i32) {
        let node = Box::new(Node {
            elem,
            next: self.head.take(),
        });
        // this node is the new head
        self.head = Some(node);
    }

    // pop either return the element or none
    pub fn pop(&mut self) -> Option<i32> {
        let head = self.head.take();
        match head {
            None => None,
            Some(node) => {
                let result = Some(node.elem);
                self.head = node.next;
                result
            }
        }
    }
    pub fn peek(&self) -> Option<&i32> {
        self.head.as_ref().map(|node| &node.elem)
    }
}

impl Default for List {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn basics() {
        let mut list = List::new();
        assert_eq!(None, list.pop());

        list.push(1);
        list.push(2);
        list.push(3);
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);

        assert_eq!(list.peek(), None);
    }
}
