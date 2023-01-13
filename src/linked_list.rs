use std::{fmt::Debug, ptr::NonNull};

type Link<T> = Option<NonNull<Node<T>>>;

struct Node<T> {
  val: T,
  prev: Link<T>,
  next: Link<T>,
}

impl<T> Node<T> {
  fn new(val: T) -> Self {
    Self {
      val,
      prev: None,
      next: None,
    }
  }
}

pub struct LinkedList<T> {
  head: Link<T>,
  tail: Link<T>,
  size: usize,
}

impl<T> LinkedList<T> {
  pub fn new() -> Self {
    Self {
      head: None,
      tail: None,
      size: 0,
    }
  }

  pub fn insert_at_head(&mut self, val: T) {
    let mut new_node = Box::new(Node::new(val));
    new_node.next = self.head;
    new_node.prev = None;
    let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(new_node)) });
    match self.head {
      Some(head_ptr) => unsafe {
        (*head_ptr.as_ptr()).prev = node_ptr;
      },
      None => self.tail = node_ptr,
    }
    self.head = node_ptr;
    self.size += 1;
  }

  pub fn insert_at_tail(&mut self, val: T) {
    let mut new_node = Box::new(Node::new(val));
    new_node.next = None;
    new_node.prev = self.tail;
    let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(new_node)) });
    match self.tail {
      Some(tail_ptr) => unsafe {
        (*tail_ptr.as_ptr()).next = node_ptr;
      },
      None => self.head = node_ptr,
    }
    self.tail = node_ptr;
    self.size += 1;
  }

  pub fn insert_at_ith(&mut self, val: T, idx: usize) {
    let size = self.size;
    if idx > size {
      panic!("index out of range");
    }
    if idx == 0 || self.head.is_none() {
      self.insert_at_head(val);
      return;
    }
    if idx == self.size {
      self.insert_at_tail(val);
      return;
    }
    let mut cur_node = self.head;
    for _ in 0..idx {
      cur_node = unsafe { (*cur_node.unwrap().as_ptr()).next };
    }
    let cur_node = cur_node.unwrap();

    let mut new_node = Box::new(Node::new(val));
    unsafe {
      new_node.prev = (*cur_node.as_ptr()).prev;
      new_node.next = Some(cur_node);
      if let Some(p) = (*cur_node.as_ptr()).prev {
        let node_ptr = Some(NonNull::new_unchecked(Box::into_raw(new_node)));
        (*p.as_ptr()).next = node_ptr;
        (*cur_node.as_ptr()).prev = node_ptr;
        self.size += 1;
      }
    }
  }

  pub fn delete_head(&mut self) -> Option<T> {
    self.head.map(|head_ptr| unsafe {
      let old_head = Box::from_raw(head_ptr.as_ptr());
      match old_head.next {
        Some(next_ptr) => (*next_ptr.as_ptr()).prev = None,
        None => self.tail = None,
      }
      self.head = old_head.next;
      self.size -= 1;
      old_head.val
    })
  }
  pub fn delete_tail(&mut self) -> Option<T> {
    self.tail.map(|tail_ptr| unsafe {
      let old_tail = Box::from_raw(tail_ptr.as_ptr());
      match old_tail.prev {
        Some(prev_ptr) => (*prev_ptr.as_ptr()).next = None,
        None => self.head = None,
      }
      self.tail = old_tail.prev;
      self.size -= 1;
      old_tail.val
    })
  }
  pub fn delete_ith(&mut self, idx: usize) -> Option<T> {
    let size = self.size;
    if idx >= size {
      panic!("index out of range");
    }
    if idx == 0 || self.head.is_none() {
      return self.delete_head();
    }
    if idx == self.size - 1 {
      return self.delete_tail();
    }
    let ith_node = self.get_ith_node(idx).unwrap();
    unsafe {
      let prev = (*ith_node.as_ptr()).prev.unwrap();
      let next = (*ith_node.as_ptr()).next.unwrap();
      (*prev.as_ptr()).next = Some(next);
      (*next.as_ptr()).prev = Some(prev);
      self.size -= 1;
      Some(Box::from_raw(ith_node.as_ptr()).val)
    }
  }

  pub fn get_ith(&self, idx: usize) -> Option<&T> {
    let ith_node = Self::get_ith_node(self, idx).unwrap();
    unsafe { Some(&(*ith_node.as_ptr()).val) }
  }

  fn get_ith_node(&self, idx: usize) -> Link<T> {
    let size = self.size;
    if idx >= size {
      panic!("index out of range");
    }
    let mut cur_node = self.head;
    for _ in 0..idx {
      cur_node = unsafe { (*cur_node.unwrap().as_ptr()).next };
    }
    cur_node
  }
}

impl<T: Debug> Debug for LinkedList<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut cur_node = self.head;
    let mut s = String::new();
    while let Some(node) = cur_node {
      unsafe {
        if cur_node != self.tail {
          s.push_str(&format!("{:?}->", (*node.as_ptr()).val));
        } else {
          s.push_str(&format!("{:?}\n", (*node.as_ptr()).val));
        }
        cur_node = (*node.as_ptr()).next;
      }
    }
    write!(f, "{}", s)
  }
}

impl<T> Drop for LinkedList<T> {
  fn drop(&mut self) {
    while self.delete_head().is_some() {}
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_linked_list() {
    let mut list = LinkedList::new();
    list.insert_at_head(2);
    // list.insert_at_ith(1, 0);
    // list.insert_at_ith(4, 2);
    // list.insert_at_ith(3, 2);
    let res = list.get_ith(0);
    println!("{:?}", res);
    print!("list: {:?}", list);
  }
}
