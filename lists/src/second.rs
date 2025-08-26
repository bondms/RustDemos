pub struct List<T> {
  head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
  elem: T,
  next: Link<T>,
}

impl<T> List<T> {
  pub fn new() -> Self {
    List{ head: None }
  }

  pub fn push(&mut self, elem: T) {
    let new_node = Box::new(Node {
      elem: elem,
      next: self.head.take(),
    });

    self.head = Some(new_node);
  }

  pub fn pop(&mut self) -> Option<T> {
    self.head.take().map(|node| {
      self.head = node.next;
      node.elem
    })
  }

  pub fn peek(&self) -> Option<&T> {
    self.head.as_ref().map(|node| {
      &node.elem
    })
  }
}

// Custom `Drop` to avoid unbounded recursion.
impl<T> Drop for List<T> {
  fn drop(&mut self) {
    let mut cur_link = self.head.take();
    while let Some(mut boxed_node) = cur_link {
      cur_link = boxed_node.next.take();
      // `boxed_node` implicitly dropped as it leaves scope.
    }
  }
}

#[cfg(test)]
mod test {
  use super::List;

  #[test]
  fn basics() {
    let mut list = List::new();

    // Empty list.
    assert_eq!(list.pop(), None);

    // Populate.
    list.push(1);
    list.push(2);
    list.push(3);

    // Removal.
    assert_eq!(list.pop(), Some(3));
    assert_eq!(list.pop(), Some(2));

    // Change again.
    list.push(4);
    list.push(5);

    // Remove to exhaustion.
    assert_eq!(list.pop(), Some(5));
    assert_eq!(list.pop(), Some(4));
    assert_eq!(list.pop(), Some(1));
    assert_eq!(list.pop(), None);
  }
}
