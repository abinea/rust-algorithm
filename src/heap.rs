use std::{cmp::Ordering, fmt::Debug};

pub enum HeapType {
  Min,
  Max,
}
pub struct Heap<T> {
  data: Vec<T>,
  cmp_fn: fn(&T, &T) -> Ordering,
}

impl<T: Ord> Heap<T> {
  pub fn new(heap_type: HeapType) -> Self {
    let cmp_fn = match heap_type {
      HeapType::Min => |_a: &T, _b: &T| _a.cmp(_b),
      HeapType::Max => |_a: &T, _b: &T| _b.cmp(_a),
    };
    Self {
      data: Vec::new(),
      cmp_fn,
    }
  }
  pub fn push(&mut self, val: T) {
    self.data.push(val);
    self.sift_up(self.data.len() - 1);
  }
  pub fn pop(&mut self) -> Option<T> {
    if self.data.is_empty() {
      return None;
    }
    let val = self.data.swap_remove(0);
    self.sift_down(0);
    Some(val)
  }
  pub fn peek(&self) -> Option<&T> {
    self.data.get(0)
  }
  pub fn is_empty(&self) -> bool {
    self.data.is_empty()
  }
  fn sift_up(&mut self, mut i: usize) {
    while i > 0 {
      let parent = (i - 1) / 2;
      if (self.cmp_fn)(&self.data[i], &self.data[parent]) == Ordering::Less {
        self.data.swap(i, parent);
        i = parent;
      } else {
        break;
      }
    }
  }
  fn sift_down(&mut self, mut i: usize) {
    while i < self.data.len() {
      let left = i * 2 + 1;
      let right = i * 2 + 2;
      let mut min = i;
      if left < self.data.len()
        && (self.cmp_fn)(&self.data[left], &self.data[min]) == Ordering::Less
      {
        min = left;
      }
      if right < self.data.len()
        && (self.cmp_fn)(&self.data[right], &self.data[min]) == Ordering::Less
      {
        min = right;
      }
      if min == i {
        break;
      }
      self.data.swap(i, min);
      i = min;
    }
  }

  pub fn size(&self) -> usize {
    self.data.len()
  }
}

impl<T: Debug> Debug for Heap<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self.data)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_heap() {
    let mut heap = Heap::new(HeapType::Max);
    heap.push(1);
    heap.push(2);
    heap.push(3);
    println!("{:?}", heap);
    assert_eq!(heap.pop(), Some(3));
  }
}
