use std::mem;

struct Node {
    elem: i32, // simple list that only stores integers
    next: Option<Box<Node>>,
}

pub struct List {
    head: Option<Box<Node>>,
}

impl List {
    pub fn new() -> Self {
        List { head: None } // return empty list
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            // references must always be valid, so if we are moving ownership of what self.head
            // currently points to to "next", we need to replace it with something else (in this
            // case, none), using the mem::replace operation
            next: mem::replace(&mut self.head, None),
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        let ret_val;

        // Note: same issue as before, we can't give "node" ownership of self.head without
        // replacing self.head with something else
        if let Some(node) = mem::replace(&mut self.head, None) {
            ret_val = Some(node.elem);
            self.head = node.next;
        } else {
            ret_val = None;
        }

        ret_val
    }
}

impl Drop for List {
    fn drop(&mut self) {
        //println!("!!! dropping !!!");
        let mut cur_node = mem::replace(&mut self.head, None);
        while let Some(mut node) = cur_node {
            //println!("dropping node containing {}", node.elem);
            cur_node = mem::replace(&mut node.next, None);
        }
    }
}

#[cfg(test)]
mod test{
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check that popping from empty list returns None
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push more items
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn cleanup() {
        {
            let mut list = List::new();
            
            list.push(1);
            list.push(2);
            list.push(3);
            list.push(4);
        }
        println!("list is now out of scope.");
    }
}
