#[allow(dead_code)]
struct Solution;

impl Solution {
  #[allow(unused)]
  pub fn is_valid(s: String) -> bool {
    let mut stack = Vec::new();
    for c in s.chars() {
      match c {
        '(' | '{' | '[' => stack.push(c),
        ')' | '}' | ']' => {
          if stack.is_empty() {
            return false;
          }
          let top = stack.pop().unwrap();
          let valid = || "([{".find(top) == ")]}".find(c);
          if !valid() {
            return false;
          }
        }
        _ => (),
      }
    }
    stack.is_empty()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_is_valid() {
    assert_eq!(Solution::is_valid("()".to_string()), true);
    assert_eq!(Solution::is_valid("()[]{}".to_string()), true);
    assert_eq!(Solution::is_valid("]".to_string()), false);
    assert_eq!(Solution::is_valid("([)]".to_string()), false);
    assert_eq!(Solution::is_valid("{[]}".to_string()), true);
  }
}
