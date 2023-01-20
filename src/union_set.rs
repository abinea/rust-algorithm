pub struct UnionSet {
  parent: Vec<usize>,
  size: Vec<usize>,
  count: usize,
}

impl UnionSet {
  pub fn new(n: usize) -> Self {
    let mut parent = vec![0; n];
    let size = vec![1; n];
    for i in 0..n {
      parent[i] = i;
    }
    Self {
      parent,
      size,
      count: n,
    }
  }

  pub fn find(&mut self, x: usize) -> usize {
    if self.parent[x] != x {
      self.parent[x] = self.find(self.parent[x]);
    }
    self.parent[x]
  }

  pub fn union(&mut self, x: usize, y: usize) {
    let root_x = self.find(x);
    let root_y = self.find(y);
    if root_x == root_y {
      return;
    }
    if self.size[root_x] < self.size[root_y] {
      self.parent[root_x] = root_y;
      self.size[root_y] += self.size[root_x];
    } else {
      self.parent[root_y] = root_x;
      self.size[root_x] += self.size[root_y];
    }
    self.count -= 1;
  }

  pub fn is_connected(&mut self, x: usize, y: usize) -> bool {
    self.find(x) == self.find(y)
  }

  pub fn count(&self) -> usize {
    self.count
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_union_set() {
    let mut u = UnionSet::new(10);
    u.union(0, 2);
    u.union(1, 2);
    u.union(2, 3);
    u.union(3, 4);
    u.union(5, 6);
    u.union(6, 7);
    u.union(7, 8);
    u.union(8, 9);
    assert_eq!(u.is_connected(1, 4), true);
    assert_eq!(u.is_connected(1, 5), false);
    assert_eq!(u.count(), 2);
  }
}
