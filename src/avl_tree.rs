use std::{
  cmp::{max, Ordering},
  mem,
  ops::Not,
};

#[derive(Clone, Copy)]
enum Side {
  Left,
  Right,
}

impl Not for Side {
  type Output = Side;
  fn not(self) -> Self::Output {
    match self {
      Side::Left => Side::Right,
      Side::Right => Side::Left,
    }
  }
}

#[derive(Debug)]
struct AVLNode<T: Ord> {
  val: T,
  left: Option<Box<AVLNode<T>>>,
  right: Option<Box<AVLNode<T>>>,
  height: usize,
}

impl<T: Ord> AVLNode<T> {
  fn rebalance(&mut self) {
    self.update_height();
    let side = match self.balance_factor() {
      -2 => Side::Left,
      2 => Side::Right,
      _ => return,
    };
    let subtree = self.child(side).as_mut().unwrap();
    if let (Side::Left, 1) | (Side::Right, -1) = (side, subtree.balance_factor()) {
      subtree.rotate(side);
    }
    self.rotate(!side);
  }

  fn update_height(&mut self) {
    self.height = 1 + max(height(&self.left), height(&self.right))
  }

  fn balance_factor(&self) -> i8 {
    let left = height(&self.left);
    let right = height(&self.right);
    if left < right {
      (right - left) as i8
    } else {
      -((left - right) as i8)
    }
  }

  fn child(&mut self, side: Side) -> &mut Option<Box<AVLNode<T>>> {
    match side {
      Side::Left => &mut self.left,
      Side::Right => &mut self.right,
    }
  }

  fn rotate(&mut self, side: Side) {
    let mut subtree = self.child(!side).take().unwrap();
    *self.child(!side) = subtree.child(side).take();
    self.update_height();
    mem::swap(self, &mut subtree);
    *self.child(side) = Some(subtree);
    self.update_height();
  }
}

fn height<T: Ord>(x: &Option<Box<AVLNode<T>>>) -> usize {
  match x {
    None => 0,
    Some(node) => node.height,
  }
}

#[derive(Debug)]
pub struct AVLTree<T: Ord> {
  root: Option<Box<AVLNode<T>>>,
  length: usize,
}

impl<T: Ord> AVLTree<T> {
  pub fn new() -> Self {
    Self {
      root: None,
      length: 0,
    }
  }

  pub fn insert(&mut self, val: T) -> bool {
    let inserted = insert(&mut self.root, val);
    if inserted {
      self.length += 1
    }
    inserted
  }
}

fn insert<T: Ord>(tree: &mut Option<Box<AVLNode<T>>>, val: T) -> bool {
  if let Some(node) = tree {
    let inserted = match val.cmp(&node.val) {
      Ordering::Equal => false,
      Ordering::Less => insert(&mut node.left, val),
      Ordering::Greater => insert(&mut node.right, val),
    };
    if inserted {
      node.rebalance();
    }
    inserted
  } else {
    *tree = Some(Box::new(AVLNode {
      val,
      height: 1,
      left: None,
      right: None,
    }));
    true
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_avl_tree() {
    let mut avl_tree = AVLTree::new();
    avl_tree.insert(32);
    avl_tree.insert(14);
    avl_tree.insert(51);
    avl_tree.insert(12);
    avl_tree.insert(22);
    avl_tree.insert(17);
    avl_tree.insert(23);
    avl_tree.insert(31);
    avl_tree.insert(30);

    println!("{:?}", avl_tree);
  }
}
