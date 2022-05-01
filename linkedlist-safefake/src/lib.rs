#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Option<usize>,
}

#[derive(Debug)]
pub struct LinkedList<T> {
    head: Option<usize>,
    nodes: Vec<Option<Node<T>>>,
    free: Vec<usize>,
    len: usize,
    capacity: usize,
}

impl<T> LinkedList<T> {
    pub fn new(capacity: usize) -> Self {
        let mut free = vec![];
        for i in 0..capacity {
            free.push(i);
        }
        let mut nodes = vec![];
        for _ in 0..capacity {
            nodes.push(None);
        }
        LinkedList {
            head: None,
            nodes,
            len: 0,
            free,
            capacity,
        }
    }

    pub fn push_front(&mut self, value: T) {
        // assert!(self.len < self.capacity);
        // let node = Node {
        //     value,
        //     next: self.head,
        // };
        // let free_index = self.free.pop().unwrap();
        // self.nodes[free_index] = Some(node);
        // self.head = Some(free_index);
        // self.len += 1;
        self.insert_next_at(None, value)
    }

    pub fn pop_front(&mut self) -> Option<T> {
        // if self.len == 0 {
        //     return None;
        // }
        // let original_head = self.head.unwrap();
        // let node = self.nodes[original_head].take().unwrap();
        // self.head = node.next;
        // self.len -= 1;
        // self.free.push(original_head);
        // Some(node.value)
        self.remove_next_at(None)
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            llist: self,
            next: self.head,
        }
    }

    pub fn into_iter(&mut self) -> IntoIter<T> {
        IntoIter { llist: self }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        let head = self.head;
        IterMut {
            llist: self,
            next: head,
        }
    }

    fn insert_next_at(&mut self, current: Option<usize>, value: T) {
        assert!(self.len < self.capacity);
        let next = match current {
            None => self.head,
            Some(current) => self.nodes[current].as_ref().unwrap().next,
        };
        let node = Node { value, next };
        let free_index = self.free.pop().unwrap();
        self.nodes[free_index] = Some(node);
        match current {
            None => self.head = Some(free_index),
            Some(current) => {
                self.nodes[current].as_mut().unwrap().next = Some(free_index);
            }
        }
        self.len += 1;
    }

    fn remove_next_at(&mut self, current: Option<usize>) -> Option<T> {
        let next = match current {
            None => self.head,
            Some(current) => self.nodes[current].as_ref().unwrap().next,
        };
        match next {
            None => None,
            Some(next) => {
                let node = self.nodes[next].take().unwrap();
                self.len -= 1;
                self.free.push(next);
                match current {
                    None => self.head = node.next,
                    Some(current) => {
                        self.nodes[current].as_mut().unwrap().next = node.next;
                    }
                }
                Some(node.value)
            }
        }
    }

    // pub fn insert_next(&mut self, iter: &mut IterMut<T>, value: T) {
    //     let current = iter.current;
    //     self.insert_next_at(current, value);
    // }

    // pub fn remove_next(&mut self, iter: &mut IterMut<T>) -> Option<T> {
    //     let current = iter.current;
    //     self.remove_next_at(current)
    // }

    pub fn cursor_mut(&mut self) -> CursorMut<T> {
        CursorMut {
            llist: self,
            current: None,
            // index: None,
        } // starting at the "ghost" element
    }
}

pub struct Iter<'a, T: 'a> {
    llist: &'a LinkedList<T>,
    next: Option<usize>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next {
            None => None,
            Some(next) => {
                let node = self.llist.nodes[next].as_ref().unwrap();
                self.next = node.next;
                Some(&node.value)
            }
        }
    }
}

pub struct IntoIter<'a, T: 'a> {
    llist: &'a mut LinkedList<T>,
}

impl<'a, T> Iterator for IntoIter<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.llist.pop_front()
    }
}

pub struct IterMut<'a, T: 'a> {
    llist: &'a mut LinkedList<T>,
    next: Option<usize>,
}

// impl<'a, T> IterMut<'a, T> {
//     pub fn insert_next(&mut self, value: T) {
//         let current = self.current;
//         self.llist.insert_next_at(current, value);
//     }

//     pub fn remove_next(&mut self) -> Option<T> {
//         let current = self.current;
//         self.llist.remove_next_at(current)
//     }
// }

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next {
            None => None,
            Some(next) => {
                // let mut node = self.nodes[next].as_mut().unwrap();
                // self.next = node.next;
                // Some(&mut node.value)
                let node = &mut self.llist.nodes[next];
                let node = node.as_mut().unwrap();
                self.next = node.next;
                let value_ptr = &mut node.value as *mut T;
                unsafe { Some(&mut *value_ptr as &mut T) }
            }
        }
    }
}

pub struct CursorMut<'a, T> {
    llist: &'a mut LinkedList<T>,
    current: Option<usize>,
    // index: Option<usize>,
}

impl<'a, T> CursorMut<'a, T> {
    pub fn move_next(&mut self) {
        match self.current {
            None => self.current = self.llist.head,
            Some(current) => {
                let node = self.llist.nodes[current].as_ref().unwrap();
                self.current = node.next;
            }
        }
    }

    pub fn current(&mut self) -> Option<&mut T> {
        match self.current {
            None => None,
            Some(current) => Some(&mut self.llist.nodes[current].as_mut().unwrap().value),
        }
    }

    pub fn peek_next(&mut self) -> Option<&mut T> {
        let next = match self.current {
            None => self.llist.head,
            Some(current) => self.llist.nodes[current].as_ref().unwrap().next,
        };
        match next {
            None => None,
            Some(next) => Some(&mut self.llist.nodes[next].as_mut().unwrap().value),
        }
    }

    pub fn insert_next(&mut self, value: T) {
        let current = self.current;
        self.llist.insert_next_at(current, value);
    }

    pub fn remove_next(&mut self) -> Option<T> {
        let current = self.current;
        self.llist.remove_next_at(current)
    }
}

#[cfg(test)]
mod tests {
    use crate::LinkedList;

    fn init_llist() -> LinkedList<i32> {
        let mut llist = LinkedList::new(10);
        llist.push_front(1);
        llist.push_front(2);
        llist.push_front(3);
        llist
    }

    #[test]
    fn push_and_pop_front() {
        let mut llist = LinkedList::<i32>::new(4);
        llist.push_front(1);
        llist.push_front(2);
        llist.push_front(3);
        llist.push_front(4);
        assert_eq!(llist.pop_front(), Some(4));
        assert_eq!(llist.pop_front(), Some(3));
        assert_eq!(llist.pop_front(), Some(2));
        assert_eq!(llist.pop_front(), Some(1));
    }

    #[test]
    fn iter() {
        let llist = init_llist();
        let mut iter = llist.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn into_iter() {
        let mut llist = init_llist();
        let mut iter = llist.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_mut() {
        let mut llist = init_llist();
        for i in llist.iter_mut() {
            *i += 1;
        }
        let mut iter = llist.iter_mut();
        assert_eq!(iter.next(), Some(&mut 4));
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn cursor() {
        let mut llist = init_llist();
        let mut cursor = llist.cursor_mut();
        assert_eq!(cursor.current(), None);
        assert_eq!(cursor.peek_next(), Some(&mut 3));
        cursor.move_next();
        assert_eq!(cursor.current(), Some(&mut 3));
        assert_eq!(cursor.peek_next(), Some(&mut 2));
        cursor.move_next();
        assert_eq!(cursor.current(), Some(&mut 2));
        assert_eq!(cursor.peek_next(), Some(&mut 1));
        cursor.move_next();
        assert_eq!(cursor.current(), Some(&mut 1));
        assert_eq!(cursor.peek_next(), None);
        cursor.move_next();
        assert_eq!(cursor.current(), None);
        assert_eq!(cursor.peek_next(), Some(&mut 3));
    }

    #[test]
    fn cursor_insert_remove() {
        let mut llist = init_llist();
        let mut cursor = llist.cursor_mut();
        cursor.move_next();
        cursor.insert_next(4); // 3 -> 4 -> 2 -> 1
        assert_eq!(cursor.peek_next(), Some(&mut 4));
        cursor.move_next();
        assert_eq!(cursor.current(), Some(&mut 4));
        assert_eq!(cursor.remove_next(), Some(2));
        assert_eq!(cursor.remove_next(), Some(1));
        assert_eq!(cursor.remove_next(), None);
        cursor.move_next(); // "ghost start point"
        assert_eq!(cursor.remove_next(), Some(3));
        assert_eq!(cursor.remove_next(), Some(4));
        assert_eq!(cursor.remove_next(), None);
        assert_eq!(llist.len, 0);
    }
}
