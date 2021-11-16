use std::cell::{Ref, RefCell};
use std::rc::Rc;

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Debug)]
pub struct Node<T: std::fmt::Debug> {
    data: T,
    prev: Link<T>,
    next: Link<T>,
}

impl<T: std::fmt::Debug> Node<T> {
    fn new(data: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            data,
            prev: None,
            next: None,
        }))
    }
}

pub struct List<T: std::fmt::Debug> {
    pub head: Link<T>,
    pub tail: Link<T>,
}

impl<T: std::fmt::Debug> List<T> {
    pub fn new() -> Self {
        return List {
            head: None,
            tail: None,
        };
    }

    pub fn push_front(&mut self, data: T) {
        let new_head = Node::new(data);

        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(Rc::clone(&new_head));
                new_head.borrow_mut().next = Some(Rc::clone(&old_head));
                self.head = Some(new_head);
            }
            None => {
                self.tail = Some(Rc::clone(&new_head));
                self.head = Some(new_head);
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    new_head.borrow_mut().prev.take();
                    self.head = Some(new_head);
                }
                None => {
                    self.tail.take();
                }
            }
            Rc::try_unwrap(old_head)
                .expect("pop_front: couldn't unwrap")
                .into_inner()
                .data
        })
    }

    pub fn peek_front(&self) -> Option<std::cell::Ref<Node<T>>> {
        self.head.as_deref().map(|v| v.borrow())
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_ref().map(|v| v.as_ptr() as *const Node<T>),
        }
    }

    pub fn iter_mut(&self) -> IterMut<T> {
        IterMut {
            next: self.head.as_ref().map(|v| v.as_ptr()),
        }
    }
}

impl<T: std::fmt::Debug> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(node) = cur_link {
            cur_link = node.borrow_mut().next.take();
        }
    }
}

pub struct Iter<T: std::fmt::Debug> {
    next: Option<*const Node<T>>,
}

impl<T: std::fmt::Debug> Iterator for Iter<T> {
    type Item = *const T;
    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            self.next.map(|node| {
                self.next = (*node)
                    .next
                    .as_deref()
                    .map(|v| v.as_ptr() as *const Node<T>);
                &(*node).data as *const T
            })
        }
    }
}

pub struct IterMut<T: std::fmt::Debug> {
    next: Option<*mut Node<T>>,
}

impl<T: std::fmt::Debug> Iterator for IterMut<T> {
    type Item = *mut T;
    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            self.next.map(|node| {
                self.next = (*node).next.as_deref().map(|v| v.as_ptr() as *mut Node<T>);
                &mut (*node).data as *mut T
            })
        }
    }
}

pub fn test_dlist() {
    {
        let mut l = List::<u32>::new();

        for i in 1..20 {
            l.push_front(i);
        }

        unsafe {
            for i in l.iter_mut() {
                *i = 32;
            }
            for i in l.iter() {
                println!("iter i: {}", *i);
            }
        }

        /*
        unsafe {
            let mut i = l.head.as_deref().map(|v| v.as_ptr());

            loop {
                if i.is_none() {
                    break;
                }

                println!("i is: {:?}", (*i.unwrap()).data);

                i = (*i.unwrap()).next.as_deref().map(|v| v.as_ptr());
            }
        }
        */
    }
}
