pub struct IntoIter<T>(List<T>); // convert list into iterator

pub struct Iter<T> {
    next: Option<&Node<T>>,
}

pub struct List<T> {
    head: Option<Box<Node<T>>>,
}

struct Node<T> {
    elem: T, // simple list that only stores integers
    next: Option<Box<Node<T>>>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None } // return empty list
    }

    pub fn peek(&self) -> Option<&T> {
        // Use as_ref() because we don't want to move node out of head
        self.head.as_ref().map(|node| {
            &node.elem
        })

        // above approach should be equivalent to the following:
        /* match &self.head {
            None => None,
            Some(node_box) => {
                Some(&node_box.elem)
            }
        }*/
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        // Use as_mut() because we don't want to move node out of head (and do want reference to be
        // mutable
        self.head.as_mut().map(|node| {
            &mut node.elem
        })
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem: elem,
            // references must always be valid, so if we are moving ownership of what self.head
            // currently points to to "next", we need to replace it with something else (in this
            // case, none). Here we use the take() method, which is the same as using the
            // mem::replace operation to replace it with "none"
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        // map gives us a concise way to return None if node is None, and execute the given closure
        // (anonymous function) on node otherwise (and then return Some(node.elem))
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<T> {
        Iter { next: self.head.map(|node| &node) }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        //println!("!!! dropping !!!");
        let mut cur_node = self.head.take();
        while let Some(mut node) = cur_node {
            //println!("dropping node containing {}", node.elem);
            cur_node = node.next.take();
        }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        // access fields of a tuple struct numerically
        self.0.pop()
    }
}

impl<T> Iterator for Iter<T> {
    type Item = &T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.map(|node| &node);
            &node.elem
        })
    }

#[cfg(test)]
mod test{
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check that peeking empty list gives None
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);

        // Check that popping from empty list returns None
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal peek
        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));
        list.peek_mut().map(|value| {
            *value = 42
        });
        assert_eq!(list.peek(), Some(&42));

        // Check normal removal
        assert_eq!(list.pop(), Some(42));
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

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

}
