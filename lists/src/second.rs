pub struct List {
  head: Link,
}

type Link = Option<Box<Node>>;

struct Node {
  elem: i32,
  next: Link,
}

impl List {
  pub fn new() -> Self {
    List{ head: None }
  }

  pub fn push(&mut self, elem: i32) {
    let new_node = Box::new(Node {
      elem: elem,
      next: self.head.take(),
    });

    self.head = Some(new_node);
  }

  pub fn pop(&mut self) -> Option<i32> {
    match self.head.take() {
      None => None,
      Some(node) => {
        self.head = node.next;
        Some(node.elem)
      }
    }
  }
}

// Custom `Drop` to avoid unbounded recursion.
impl Drop for List {
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
