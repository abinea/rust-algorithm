use super::linked_list::LinkedList;

#[derive(Debug, Default)]
pub struct Stack<T> {
  list: LinkedList<T>,
}

impl<T> Stack<T> {
  pub fn new() -> Self {
    Self {
      list: LinkedList::new(),
    }
  }

  pub fn push(&mut self, val: T) {
    self.list.insert_at_tail(val);
  }

  pub fn pop(&mut self) -> Option<T> {
    self.list.delete_tail()
  }

  pub fn peek(&self) -> Option<&T> {
    self.list.get_ith(0)
  }

  pub fn is_empty(&self) -> bool {
    self.list.is_empty()
  }

  pub fn size(&self) -> usize {
    self.list.size()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_stack_using_linked_list() {
    let mut s: Stack<i32> = Stack::new();
    s.push(1);
    s.push(2);
    s.pop();
    s.pop();
    s.pop();
    assert_eq!(s.is_empty(), true);
  }
}
