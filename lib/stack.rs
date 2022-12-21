#[derive(Debug)]
struct Stack<T> {
  top: usize,   // 栈顶index
  data: Vec<T>, // 数据
}

impl<T> Stack<T> {
  fn new() -> Self {
    Self {
      top: 0,
      data: Vec::new(),
    }
  }
  fn push(&mut self, item: T) {
    self.data.push(item);
    self.top += 1;
  }

  fn pop(&mut self) -> Option<T> {
    if self.is_empty() {
      return None;
    }
    self.top -= 1;
    Some(self.data.pop().unwrap())
  }

  fn is_empty(&self) -> bool {
    self.top == 0
  }

  fn peek(&self) -> Option<&T> {
    match self.is_empty() {
      true => None,
      false => Some(&self.data[self.top - 1]),
    }
  }

  fn size(&self) -> usize {
    self.top
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn it_works() {
    let mut s:Stack<i32> = Stack::new();
    s.push(1);
    s.push(2);
    s.push(3);
    let top = s.pop();
    assert_eq!(top.unwrap(), 3);
    assert_eq!(s.size(), 2);
    assert_eq!(s.peek().unwrap(), &2);
  }
}
