use std::mem;

struct Node {
    elem: i32, // simple list that only stores integers
    next: Option<Box<Node>>,
}

pub struct List {
    head: Option<Box<Node>>,
    len: u32,
}

impl List {
    pub fn new() -> Self {
        List { head: None, len: 0 } // return empty list
    }

    // Print the contents of the list
    pub fn print(&mut self) {
        println!("length: {}", self.len);

        let mut cur_node = &self.head;
        while let Some(node) = cur_node {
            print!("{} -> ", node.elem);
            cur_node = &node.next;
        }
        println!("end");
    }

    // Insert a number into the repository. Each number should only appear at most once in the
    // repository. The operation should return True if the number was inserted, False if the number
    // was not inserted, and an Error if the operation had a problem completing correctly.
    pub fn insert(&mut self, elem: i32) -> bool {
        // Empty list, just add as only element
        if self.head.is_none() {
            let new_node = Box::new(Node { elem: elem, next: None });
            self.head = Some(new_node);
            self.len += 1;
            return true;
        }

        // Special case to allow checking head itself
        if let Some(node) = &mut self.head {
            if elem == node.elem {
                return false;
            } else if elem < node.elem {
                // Value to insert is less than head; add new node before current head (and set
                // head to new node)
                let new_node = Box::new(Node { elem: elem, next: mem::replace(&mut self.head, None) });
                self.head = Some(new_node);
                self.len += 1;
                return true;
            }
        }

        // General case: check whether elem is less than the value of the *next* node. If it is,
        // insert between current and next node
        let mut cur_opt = &mut self.head;
        while let Some(ref mut cur_node) = cur_opt { // TODO: understand "ref" in more detail
            if let Some(next_node) = &cur_node.next {
                if elem == next_node.elem {
                    return false;
                } else if elem < next_node.elem {
                    // Value to insert is less than next nodes's value; add new node between
                    // cur_node and next_node
                    let new_node = Box::new(Node { elem: elem, next: mem::replace(&mut cur_node.next, None) });
                    cur_node.next = Some(new_node);
                    self.len += 1;
                    return true;
                }
                cur_opt = &mut cur_node.next;
            } else { // next_node is null
                // We traversed the whole list and this value was greater than every element. Add at end.
                let new_node = Box::new(Node { elem: elem, next: None });
                cur_node.next = Some(new_node);
                self.len += 1;
                return true;
            }
        }

        unreachable!();
    }

    // Remove a number from the repository if such exists. The operation should return True if the
    // number was removed, and False if the number was not found in the repository.
    pub fn delete(&mut self, elem: i32) -> bool {
        // Empty list, nothing to do
        if self.head.is_none() {
            return false;
        }

        // Special case to allow checking head itself
        if let Some(node) = &mut self.head {
            if elem == node.elem {
                self.head = mem::replace(&mut node.next, None);
                self.len -= 1;
                return true;
            } else if elem < node.elem {
                return false;
            }
        }
        // At this point, we know value to remove is greater than the value of the head

        // General case: check whether elem is equal to the value of the *next* node. if it is,
        // remove and re-link. if we reach a larger value, can stop searching
        let mut cur_opt = &mut self.head;
        while let Some(ref mut cur_node) = cur_opt { // TODO: understand "ref" in more detail
            if let Some(next_node) = &mut cur_node.next {
                if elem == next_node.elem {
                    cur_node.next = mem::replace(&mut next_node.next, None);
                    self.len -= 1;
                    return true;
                } else if elem < next_node.elem {
                    return false;
                }
                cur_opt = &mut cur_node.next;
            } else { // next_node is null
                // We traversed the whole list and this value was greater than every element. Nothing to do
                return false;
            }
        }

        unreachable!();
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

        // Inserting into empty list should work (return true)
        assert_eq!(list.insert(10), true);

        // Inserting same element to list with single element should return false
        assert_eq!(list.insert(10), false);

        // Inserting value greater than 10 should work (but disallow duplicates)
        assert_eq!(list.insert(20), true);

        // Inserting values less than 10 should work
        assert_eq!(list.insert(9), true);
        assert_eq!(list.insert(8), true);

        list.print();

        // Inserting between existing values should work
        assert_eq!(list.insert(11), true);
        assert_eq!(list.insert(12), true);

        // Inserting at end should work
        assert_eq!(list.insert(21), true);

        // Inserting values that already exist should fail
        assert_eq!(list.insert(9), false);
        assert_eq!(list.insert(8), false);
        assert_eq!(list.insert(21), false);

        // Inserting at beginning and end should still work
        assert_eq!(list.insert(7), true);
        assert_eq!(list.insert(22), true);

        list.print();

        // Deleting from beginning and end should work
        assert_eq!(list.delete(7), true);
        assert_eq!(list.delete(22), true);

        // Deleting from middle should work
        assert_eq!(list.delete(11), true);
        assert_eq!(list.delete(12), true);

        // Deleting already deleted elements should fail
        assert_eq!(list.delete(7), false);
        assert_eq!(list.delete(22), false);
        assert_eq!(list.delete(12), false);

        list.print();

        let mut list2 = List::new();

        // Deleting from empty list should fail
        assert_eq!(list2.delete(1), false);

        assert_eq!(list2.insert(1), true);

        // Deleting only element from list should work
        assert_eq!(list2.delete(1), true);

        assert_eq!(list2.insert(1), true);
        assert_eq!(list2.insert(2), true);
        assert_eq!(list2.delete(1), true);
        assert_eq!(list2.delete(1), false);

        list2.print();
    }
}
