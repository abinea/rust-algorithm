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

// 取非逻辑
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
  // 平衡当前节点子树
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
  // 更新节点高度
  fn update_height(&mut self) {
    self.height = 1 + max(height(&self.left), height(&self.right))
  }
  // 平衡因子计算
  fn balance_factor(&self) -> i8 {
    let left = height(&self.left);
    let right = height(&self.right);
    if left < right {
      (right - left) as i8
    } else {
      -((left - right) as i8)
    }
  }
  // 获取子树可变引用
  fn child(&mut self, side: Side) -> &mut Option<Box<AVLNode<T>>> {
    match side {
      Side::Left => &mut self.left,
      Side::Right => &mut self.right,
    }
  }
  // 旋转操作
  fn rotate(&mut self, side: Side) {
    let mut subtree = self.child(!side).take().unwrap();
    *self.child(!side) = subtree.child(side).take();
    self.update_height();
    mem::swap(self, &mut subtree);
    *self.child(side) = Some(subtree);
    self.update_height();
  }
}
// 高度计算
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

  pub fn remove(&mut self, val: T) -> bool {
    let removed = remove(&mut self.root, val);
    if removed {
      self.length -= 1
    }
    removed
  }
}

fn insert<T: Ord>(tree: &mut Option<Box<AVLNode<T>>>, val: T) -> bool {
  if let Some(node) = tree {
    let inserted = match val.cmp(&node.val) {
      Ordering::Equal => false,
      Ordering::Less => insert(&mut node.left, val),
      Ordering::Greater => insert(&mut node.right, val),
    };
    // 如果插入新节点，需要重新平衡
    if inserted {
      node.rebalance();
    }
    inserted
  } else {
    // 如果树为空，直接插入
    *tree = Some(Box::new(AVLNode {
      val,
      height: 1,
      left: None,
      right: None,
    }));
    true
  }
}

fn remove<T: Ord>(tree: &mut Option<Box<AVLNode<T>>>, val: T) -> bool {
  if let Some(node) = tree {
    let removed = match val.cmp(&node.val) {
      Ordering::Less => remove(&mut node.left, val),
      Ordering::Greater => remove(&mut node.right, val),
      Ordering::Equal => {
        *tree = match (node.left.take(), node.right.take()) {
          // 叶子节点直接删除
          (None, None) => None,
          // 只有一个子树，直接替换
          (Some(left), None) => Some(left),
          (None, Some(right)) => Some(right),
          // 有两个子树，找到右子树最小节点替换
          (Some(left), Some(right)) => Some(merge(left, right)),
        };
        return true;
      }
    };
    if removed {
      node.rebalance();
    }
    removed
  } else {
    // 如果树为空，直接返回false
    false
  }
}

/// 合并两个子树
fn merge<T: Ord>(left: Box<AVLNode<T>>, right: Box<AVLNode<T>>) -> Box<AVLNode<T>> {
  let mut op_right = Some(right);
  // 合并后的子树根节点为右子树最小节点
  let mut root = take_min(&mut op_right).unwrap();
  // 左子树为原左子树，右子树为删除最小节点后平衡的右子树
  root.left = Some(left);
  root.right = op_right;
  // 重新平衡
  root.rebalance();
  root
}

/// 取出最小节点
fn take_min<T: Ord>(tree: &mut Option<Box<AVLNode<T>>>) -> Option<Box<AVLNode<T>>> {
  if let Some(mut node) = tree.take() {
    if let Some(small) = take_min(&mut node.left) {
      // 回溯时，需要重新平衡
      node.rebalance();
      // 各个根节点重新指向平衡后的子树
      *tree = Some(node);
      // 逐级向上传最小节点
      Some(small)
    } else {
      // 用最小节点的右子树取代该节点位置，如果没有右子树，则为空
      *tree = node.right.take();
      // 返回该最小节点
      Some(node)
    }
  } else {
    None
  }
}

#[cfg(test)]
mod tests {
  use super::AVLTree;

  #[test]
  fn test_avl_tree() {
    // test insert
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
    if let Some(root) = &avl_tree.root {
      if let Some(right) = &root.right {
        if let Some(left) = &right.left {
          assert_eq!(left.val, 30);
        }
      }
    };

    println!("{:?}", &avl_tree.root);

    // test remove
    avl_tree.remove(32);
    println!("{:?}", &avl_tree.root);
  }
}
