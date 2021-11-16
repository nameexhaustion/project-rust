type Link<T> = Option<Box<Node<T>>>;

pub struct Node<T> {
    data: T,
    next: Link<T>,
}

pub struct List<T> {
    pub head: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        return List { head: None };
    }

    pub fn push(&mut self, data: T) {
        let n = Box::new(Node {
            data,
            next: self.head.take(),
        });

        self.head = Some(n);
    }

    pub fn pop(&mut self) -> Option<T> {
        if let Some(n) = self.head.take() {
            let d = n.data;
            self.head = n.next;
            return Some(d);
        }
        None
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_deref(),
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            next: self.head.as_deref_mut(),
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.data
        })
    }
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.data
        })
    }
}
