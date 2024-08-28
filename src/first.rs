use std::mem;

struct Node {
    elem: i32,
    next: Link,
}
enum Link {
    Empty,
    More(Box<Node>),
}

pub struct List {
    head: Link,
}
impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    

    pub fn push(&mut self,elem:i32){
        let node = Box::new( Node {
            elem,
            next: mem::replace(&mut self.head, Link::Empty)
        });
        // this node is the new head
        self.head = Link::More(node);
    }

    // pop either return the element or none
    pub fn pop(&mut self)-> Option<i32>{
        let head = mem::replace(&mut self.head, Link::Empty);
        match head {
            Link::Empty => None,
            Link::More(node)=>{
                let result = Some(node.elem);
                self.head = node.next;
                result
            }

        }
    }
    pub fn peek(&self)->Option<&i32>{
        match &self.head {
            Link::Empty => None,
            Link::More(node)=> Some(&node.elem)
        }
    }


}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn basics(){
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
        
        assert_eq!(list.peek(),None);

    }
}