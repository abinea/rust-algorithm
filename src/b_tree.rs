use std::{fmt::Debug, mem};

#[derive(Debug)]
struct Node<T> {
  keys: Vec<T>,
  children: Vec<Node<T>>,
}

impl<T: Ord> Node<T> {
  fn new(degree: usize, _keys: Option<Vec<T>>, _children: Option<Vec<Node<T>>>) -> Self {
    Self {
      keys: _keys.unwrap_or_else(|| Vec::with_capacity(degree - 1)),
      children: _children.unwrap_or_else(|| Vec::with_capacity(degree)),
    }
  }

  fn is_leaf(&self) -> bool {
    self.children.is_empty()
  }
}

#[derive(Debug)]
struct BTreeProps {
  degree: usize,
  max_keys: usize,
  min_key_index: usize,
}

impl BTreeProps {
  fn new(degree: usize) -> Self {
    Self {
      degree,
      max_keys: degree - 1,
      min_key_index: (degree - 1) / 2,
    }
  }

  fn is_maxed_out<T: Ord + Copy>(&self, node: &Node<T>) -> bool {
    node.keys.len() == self.max_keys
  }

  fn split_child<T: Ord + Copy + Default>(&self, parent: &mut Node<T>, child_index: usize) {
    let child = &mut parent.children[child_index];
    let middle_key = child.keys[self.min_key_index];
    let right_keys = match child.keys.split_off(self.min_key_index).split_first() {
      Some((_first, _others)) => _others.to_vec(),
      None => Vec::with_capacity(self.max_keys),
    };
    let right_children = if !child.is_leaf() {
      Some(child.children.split_off(self.min_key_index + 1))
    } else {
      None
    };
    let new_child_node: Node<T> = Node::new(self.degree, Some(right_keys), right_children);
    parent.keys.insert(child_index, middle_key);
    parent.children.insert(child_index + 1, new_child_node);
  }

  fn insert_non_full<T: Ord + Copy + Default>(&mut self, node: &mut Node<T>, key: T) {
    let mut index = isize::try_from(node.keys.len()).ok().unwrap() - 1;
    while index >= 0 && node.keys[index as usize] > key {
      index -= 1;
    }

    let mut u_index = usize::try_from(index + 1).ok().unwrap();
    if node.is_leaf() {
      node.keys.insert(u_index, key);
    } else {
      if self.is_maxed_out(&node.children[u_index]) {
        self.split_child(node, u_index);
        if node.keys[u_index] < key {
          u_index += 1;
        }
      }
      self.insert_non_full(&mut node.children[u_index], key);
    }
  }
  fn traverse_node<T: Ord + Debug>(node: &Node<T>, depth: usize) {
    if node.is_leaf() {
      print!("{0:{<1$}{2:?}{0:}<1$} ", "", depth, node.keys);
    } else {
      let _depth = depth + 1;
      for (index, key) in node.keys.iter().enumerate() {
        Self::traverse_node(&node.children[index], _depth);
        print!("{0:{<1$}{2:?}{0:}<1$} ", "", depth, key);
      }
      Self::traverse_node(node.children.last().unwrap(), _depth);
    }
  }
}

#[derive(Debug)]
struct BTree<T> {
  root: Node<T>,
  props: BTreeProps,
}

impl<T> BTree<T>
where
  T: Ord + Copy + Default + Debug,
{
  pub fn new(branch_factor: usize) -> Self {
    let degree = 2 * branch_factor;
    Self {
      root: Node::new(degree, None, None),
      props: BTreeProps::new(degree),
    }
  }

  pub fn insert(&mut self, key: T) {
    if self.props.is_maxed_out(&self.root) {
      let mut new_root = Node::new(self.props.degree, None, None);
      mem::swap(&mut new_root, &mut self.root);
      self.root.children.insert(0, new_root);
      self.props.split_child(&mut self.root, 0);
    }
    self.props.insert_non_full(&mut self.root, key);
  }

  pub fn traverse(&self) {
    BTreeProps::traverse_node(&self.root, 0);
    println!();
  }

  pub fn search(&self, key: T) -> bool {
    let mut current_node = &self.root;
    let mut index: isize;
    loop {
      index = isize::try_from(current_node.keys.len()).ok().unwrap() - 1;
      while index >= 0 && current_node.keys[index as usize] > key {
        index -= 1;
      }

      let u_index = usize::try_from(index + 1).ok().unwrap();
      if index >= 0 && current_node.keys[u_index - 1] == key {
        break true;
      } else if current_node.is_leaf() {
        break false;
      } else {
        current_node = &current_node.children[u_index];
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::BTree;
  #[test]
  fn test_b_tree() {
    let mut b_tree = BTree::new(2);
    let test_data = vec![10, 20, 30, 5, 6, 7, 11, 12, 15];
    for val in test_data {
      b_tree.insert(val);
    }
    assert!(b_tree.search(15));
    assert_eq!(b_tree.search(16), false);
    // 可视化：https://www.cs.usfca.edu/~galles/visualization/BTree.html，选择偶数degree并勾选 Preemtive Split / Merge (Even max degree only)
    println!("{:#?}", b_tree);
  }
}
