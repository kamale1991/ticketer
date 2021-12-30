use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;
use std::fmt;

type Link<T> = Rc<RefCell<Node<T>>>;

pub struct Iter<T> {
    current: Option<Link<T>>,
}

impl<T: Clone> Iterator for Iter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.current.take() {
            None => None,
            Some(curr) => {
                let curr = curr.borrow();
                let v = curr.value.clone();
                match curr.next {
                    None => {
                        self.current = None;
                    }
                    Some(ref next) => {
                        self.current = Some(Rc::clone(next));
                    }
                }
                Some(v)
            }
        }
    }
}

impl<T: Clone> DoubleEndedIterator for Iter<T> {
    fn next_back(&mut self) -> Option<T> {
        match self.current.take() {
            None => None,
            Some(curr) => {
                let curr = curr.borrow();
                match curr.prev {
                    None => {
                        self.current = None;
                        None
                    },
                    Some(ref prev) => {
                        self.current = Some(Rc::clone(prev));
                        Some(prev.borrow().value.clone())
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
struct Node<T> {
    value: T,
    prev: Option<Link<T>>,
    next: Option<Link<T>>,
}

// 以下のサイトを参考にして実装した
// https://blog.ymgyt.io/entry/2019/08/17/013313
impl<T> Node<T> {    
    fn new(value: T) -> Link<T> {
        Rc::new(RefCell::new(
            Self {
                value,
                prev: None,
                next: None,
            }
        ))
    }
}

#[derive(Default)]
pub struct LinkedList<T> {
    head: Option<Link<T>>,
    tail: Option<Link<T>>,
    length: usize,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            length: 0,
        }
    }

    pub fn len(&self) -> i32 {
        self.length as i32
    }

    pub fn append(&mut self, v: T) {
        let node = Node::new(v);
        match self.tail.take() {
            Some(old_tail) => {
                (*old_tail).borrow_mut().next = Some(Rc::clone(&node));
                (*node).borrow_mut().prev = Some(old_tail);
            },
            None => {
                self.head = Some(Rc::clone(&node));
            }
        }
        self.tail = Some(node);
        self.length += 1;        
    }

    pub fn prepend(&mut self, v: T) {
        let node = Node::new(v);
        match self.head.take() {
            None => {
                self.head = Some(Rc::clone(&node))
            },
            Some(old_head) => {                
                (*old_head).borrow_mut().prev = Some(Rc::clone(&node));
                (*node).borrow_mut().next = Some(old_head);
            }
        }
        self.head = Some(node);
        self.length += 1;
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            current: if self.len() == 0 {
                None
            } else {
                Some(Rc::clone(&self.head.as_ref().unwrap()))
            },
        }
    }

}

impl<T: fmt::Display + Clone> fmt::Debug for LinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let iter = self.iter();
        write!(f, "{{ head")?;
        for v in iter {
            write!(f, " -> {}", v)?;            
        }
        write!(f, " }}")
    }
}

/*
    pub fn get(index: i32) -> Result<T, ()> {
        match Self::head {
            None => panic!("Invalid index. There is no head node."),
            Some(_) => {
                let cur: Option<Node<T>> = Self::head; //.clone();
                for i in 0..index {                    
                    if cur.unwrap().next == None {
                        panic!("")
                    }
                    cur = cur.unwrap().next.unwrap().as_mut();
                }
                Ok(cur.val_ptr)
            }
        }
    }

    pub fn add_at_head(val_ptr: NonNull<T>) {
        if (head == Null) {
            head = Node::new_item(val_ptr);
            head;
        }
        let node: Self = Node::new_item(val_ptr);
        node.next = head;
        head.prev = node;
        head = node;
    }
}
*/

#[cfg(test)]
mod tests {
    use super::*;    

    //#[test]
    //fn test_append() {
    //    let x: i32 = 1;
    //    let ll = LinkedList::new();
    //    ll.append(x);        
    //}

    #[test]
    fn reverse() {
        let mut list: LinkedList<i32> = LinkedList::new();
        (0..10).for_each(|n| list.append(n));

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next_back(), Some(3));
        assert_eq!(iter.next_back(), Some(2));
        assert_eq!(iter.next_back(), Some(1));
        assert_eq!(iter.next_back(), Some(0));
        assert_eq!(iter.next_back(), None);
    }


}