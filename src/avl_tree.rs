use std::{
  cmp::{max, Ordering},
  fmt::Debug,
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
    let subtree = self.child_mut(side).as_mut().unwrap();
    if let (Side::Left, 1) | (Side::Right, -1) = (side, subtree.balance_factor()) {
      subtree.rotate(side);
    }
    self.rotate(!side);
  }
  // 更新节点高度
  fn update_height(&mut self) {
    self.height = 1 + max(self.height(Side::Left), self.height(Side::Right))
  }
  // 高度计算
  fn height(&self, side: Side) -> usize {
    self.child(side).as_ref().map_or(0, |n| n.height)
  }
  // 平衡因子计算
  fn balance_factor(&self) -> i8 {
    let left = self.height(Side::Left);
    let right = self.height(Side::Right);
    if left < right {
      (right - left) as i8
    } else {
      -((left - right) as i8)
    }
  }
  // 获取子树不可变引用
  fn child(&self, side: Side) -> &Option<Box<AVLNode<T>>> {
    match side {
      Side::Left => &self.left,
      Side::Right => &self.right,
    }
  }
  // 获取子树可变引用
  fn child_mut(&mut self, side: Side) -> &mut Option<Box<AVLNode<T>>> {
    match side {
      Side::Left => &mut self.left,
      Side::Right => &mut self.right,
    }
  }
  // 旋转操作
  fn rotate(&mut self, side: Side) {
    let mut subtree = self.child_mut(!side).take().unwrap();
    *self.child_mut(!side) = subtree.child_mut(side).take();
    self.update_height();
    mem::swap(self, &mut subtree);
    *self.child_mut(side) = Some(subtree);
    self.update_height();
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

  pub fn is_empty(&self) -> bool {
    self.length == 0
  }

  pub fn size(&self) -> usize {
    self.length
  }

  pub fn iter(&self) -> Iter<T> {
    let cap = self.root.as_ref().map_or(0, |n| n.height);
    let mut node_stack = Vec::with_capacity(cap);

    let mut child = &self.root;
    while let Some(node) = child {
      node_stack.push(node.as_ref());
      child = &node.left;
    }

    Iter { node_stack }
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

pub struct Iter<'a, T: Ord> {
  node_stack: Vec<&'a AVLNode<T>>,
}

impl<'a, T: Ord> Iterator for Iter<'a, T> {
  type Item = &'a T;
  fn next(&mut self) -> Option<Self::Item> {
    if let Some(node) = self.node_stack.pop() {
      // Push left path of right subtree to stack
      let mut child = &node.right;
      while let Some(subtree) = child {
        self.node_stack.push(subtree.as_ref());
        child = &subtree.left;
      }
      Some(&node.val)
    } else {
      None
    }
  }
}

#[cfg(test)]
mod tests {
  use super::AVLTree;

  #[test]
  fn test_avl_tree() {
    // test insert
    let mut avl_tree = AVLTree::new();
    let test_data = vec![32, 14, 51, 12, 22];
    for val in test_data {
      avl_tree.insert(val);
    }
    let root = avl_tree.root.as_ref().unwrap();
    assert_eq!(root.val, 32);
    let test_other = vec![23, 31, 30];
    for val in test_other {
      avl_tree.insert(val);
    }
    let root = avl_tree.root.as_ref().unwrap();
    assert_eq!(root.val, 22);
    let right = root.right.as_ref().unwrap();
    assert_eq!(right.val, 32);

    // test remove
    avl_tree.remove(12);
    println!("{:?}", &avl_tree.root);

    // test iter
    let mut iter = avl_tree.iter();
    let mut data = vec![];
    while let Some(val) = iter.next() {
      data.push(*val);
    }
    println!("{:?}", data);
  }
}
