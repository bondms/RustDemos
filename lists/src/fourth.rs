use std::cell::RefCell;
use std::rc::Rc;

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>,
}

impl<T> Node<T> {
    fn new(elem: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            elem: elem,
            prev: None,
            next: None,
        }))
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: None,
            tail: None,
        }
    }

    pub fn push_front(&mut self, elem: T) {
        let new_head = Node::new(elem);
        match self.head.take() {
            Some(old_head) => {
                // Pushing to front of non-empty list.
                old_head.borrow_mut().prev = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old_head);
                self.head = Some(new_head);
            }
            None => {
                // Pushing to front of empty list.
                self.tail = Some(new_head.clone());
                self.head = Some(new_head);
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    // Popping from front of list without emptying it.
                    new_head.borrow_mut().prev.take();
                    self.head = Some(new_head);
                }
                None => {
                    // Popping last item from front of list.
                    self.tail.take();
                }
            }
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().elem
        })
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Empty list.
        assert_eq!(list.pop_front(), None);

        // Populate.
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        // Removal.
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));

        // Rebuild.
        list.push_front(4);
        list.push_front(5);

        // Removal.
        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.pop_front(), Some(4));
        assert_eq!(list.pop_front(), Some(1));

        // Exhaustion.
        assert_eq!(list.pop_front(), None);

    }
}
