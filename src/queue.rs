use crate::linked_list::LinkedList;

#[derive(Debug, Default)]
pub struct Queue<T> {
  list: LinkedList<T>,
}

impl<T> Queue<T> {
  pub fn new() -> Self {
    Self {
      list: LinkedList::new(),
    }
  }

  pub fn enqueue(&mut self, value: T) {
    self.list.insert_at_tail(value)
  }

  pub fn dequeue(&mut self) -> Option<T> {
    self.list.delete_head()
  }

  pub fn head(&self) -> Option<&T> {
    self.list.get_ith(0)
  }

  pub fn tail(&self) -> Option<&T> {
    let size = self.list.size();
    self.list.get_ith(size - 1)
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
  fn test_queue() {
    let mut queue = Queue::new();
    queue.enqueue(1);
    queue.enqueue(2);

    queue.dequeue();
    assert_eq!(queue.size(), 1);
    assert_eq!(*queue.head().unwrap(), 2);
    queue.dequeue();

    assert_eq!(queue.size(), 0);
  }
}
