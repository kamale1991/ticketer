//use core::num::fmt::Part;
//use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;
use std::fmt;

//use num::bigint::ParseBigIntError;

type Link<T> = Rc<RefCell<Node<T>>>;

pub struct Iter<T>
where T: PartialEq,
{
    current: Option<Link<T>>,
}

impl<T: Clone> Iterator for Iter<T>
where T: PartialEq,
{
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

impl<T: Clone> DoubleEndedIterator for Iter<T>
where T: PartialEq,
{
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
struct Node<T> 
where T: PartialEq,
{
    value: T,
    prev: Option<Link<T>>,
    next: Option<Link<T>>,
}

// 以下のサイトを参考にして実装した
// https://blog.ymgyt.io/entry/2019/08/17/013313
impl<T> Node<T> 
where T: PartialEq,
{        
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

impl<T: Clone> PartialEq for Node<T> 
where T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        if self.value == other.value {
            return true
        } else {
            return false
        }
    }
}

#[derive(Default)]
pub struct LinkedList<T>
where T: PartialEq,
{
    head: Option<Link<T>>,
    tail: Option<Link<T>>,
    length: usize,
}

impl<T: Clone> PartialEq for LinkedList<T>
where T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        if self.length != other.length {
            return false
        }
        let mut iter_other = other.iter();
        let mut iter = self.iter();
        for i in 0..self.length {
            if iter_other.next() != iter.next() {
                return false
            }
        }
        true
    }
}

impl<T> LinkedList<T>
where T: PartialEq,
{
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

impl<T: fmt::Display + Clone> fmt::Debug for LinkedList<T>
where T: PartialEq,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let iter = self.iter();
        write!(f, "{{ head")?;
        for v in iter {
            write!(f, " -> {}", v)?;            
        }
        write!(f, " }}")
    }
}

impl<T: fmt::Display + Clone> fmt::Display for LinkedList<T>
where T: PartialEq,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let iter = self.iter();
        write!(f, "{{ head")?;
        for v in iter {
            write!(f, " -> {}", v)?;            
        }
        write!(f, " }}")
    }
}


#[cfg(test)]
mod tests {
    use super::*;    

    #[test]
    fn test_append() {
        let x: i32 = 1;
        let mut ll = LinkedList::new();
        ll.append(x);        
    }

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