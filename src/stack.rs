#[derive(Debug)]
pub struct Stack<T> {
  data: Vec<T>, // 数据
}

impl<T> Stack<T> {
  pub fn new() -> Self {
    Self { data: Vec::new() }
  }
  pub fn push(&mut self, item: T) {
    self.data.push(item);
  }

  pub fn pop(&mut self) -> Option<T> {
    if self.is_empty() {
      return None;
    }
    Some(self.data.pop().unwrap())
  }

  pub fn is_empty(&self) -> bool {
    self.size() == 0
  }

  pub fn peek(&self) -> Option<&T> {
    match self.is_empty() {
      true => None,
      false => Some(&self.data[self.size() - 1]),
    }
  }

  pub fn size(&self) -> usize {
    self.data.len()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_stack() {
    let mut s: Stack<i32> = Stack::new();
    s.push(1);
    s.push(2);
    s.push(3);
    let top = s.pop();
    assert_eq!(top.unwrap(), 3);
    assert_eq!(s.size(), 2);
    assert_eq!(s.peek().unwrap(), &2);
  }
}
