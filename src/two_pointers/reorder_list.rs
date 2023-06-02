#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}
impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
#[allow(dead_code)]
pub fn reorder_list(head: &mut Option<Box<ListNode>>) {}

#[cfg(test)]
mod test {
    use super::*;
    use std::assert_eq;
    #[test]
    fn test_reorder_list() {
        let n = ListNode::new(1);
    }
}
