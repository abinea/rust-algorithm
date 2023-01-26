use std::cmp::Ordering;

#[derive(Debug, Default)]
pub struct BinarySearchTree<T> {
  val: Option<T>,
  left: Option<Box<BinarySearchTree<T>>>,
  right: Option<Box<BinarySearchTree<T>>>,
}

impl<T: Ord> BinarySearchTree<T> {
  pub fn new() -> Self {
    Self {
      val: None,
      left: None,
      right: None,
    }
  }

  pub fn insert(&mut self, val: T) {
    if self.val.is_none() {
      self.val = Some(val);
    } else {
      match self.val.as_ref().unwrap().cmp(&val) {
        Ordering::Equal => return,
        Ordering::Less => {
          if self.right.is_none() {
            self.right = Some(Box::new(BinarySearchTree::new()));
          }
          self.right.as_mut().unwrap().insert(val);
        }
        Ordering::Greater => {
          if self.left.is_none() {
            self.left = Some(Box::new(BinarySearchTree::new()));
          }
          self.left.as_mut().unwrap().insert(val);
        }
      }
    }
  }

  pub fn search(&self, val: T) -> bool {
    match self.val.as_ref().unwrap().cmp(&val) {
      Ordering::Equal => true,
      Ordering::Less => match &self.right {
        None => false,
        Some(right) => right.search(val),
      },
      Ordering::Greater => match &self.left {
        None => false,
        Some(left) => left.search(val),
      },
    }
  }

  pub fn remove(&mut self, val: T) -> bool {
    match self.val.as_ref().unwrap().cmp(&val) {
      Ordering::Equal => {
        if self.left.is_none() && self.right.is_none() {
          self.val = None;
        } else if self.left.is_none() {
          self.val = self.right.as_mut().unwrap().val.take();
          self.right = self.right.as_mut().unwrap().right.take();
          self.left = self.right.as_mut().unwrap().left.take();
        } else if self.right.is_none() {
          self.val = self.left.as_mut().unwrap().val.take();
          self.left = self.left.as_mut().unwrap().left.take();
        } else {
          let mut right = self.right.take().unwrap();
          self.val = right.val.take();
          self.right = right.right.take();
          self
            .left
            .as_mut()
            .unwrap()
            .insert(right.left.take().unwrap().val.take().unwrap());
        }
        true
      }
      Ordering::Less => match &mut self.right {
        None => false,
        Some(right) => right.remove(val),
      },
      Ordering::Greater => match &mut self.left {
        None => false,
        Some(left) => left.remove(val),
      },
    }
  }
}

#[cfg(test)]
mod tests {
  use super::BinarySearchTree;
  #[test]
  fn test_bst() {
    let mut bst = BinarySearchTree::new();
    bst.insert(1);
    bst.insert(2);
    bst.insert(3);
    bst.insert(4);
    bst.insert(5);

    assert_eq!(bst.search(1), true);
    let res = bst.remove(3);
    let res = bst.remove(1);
    println!("{:#?}", bst);
    assert_eq!(bst.search(3), false);
  }
}
