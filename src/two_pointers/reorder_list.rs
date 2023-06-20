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
pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
    println!("\nstart");
    // let mut slow: &Option<Box<ListNode>> = &head.clone();
    // let mut fast: &Option<Box<ListNode>> = &head.clone();
    // let mut slow = &head.clone();
    // let mut fast = &head.clone();
    let mut slow = head.as_ref();
    let mut fast = head.as_ref();

    loop {
        println!(
            "slow={:?} fast={:?} ",
            slow.as_ref().map(|n| n.val),
            fast.as_ref().map(|n| n.val)
        );
        if let Some(_) = fast.unwrap().next {
            fast = fast.unwrap().next.as_ref();
            if let Some(_) = fast.unwrap().next {
                fast = fast.unwrap().next.as_ref();
                slow = slow.unwrap().next.as_ref();
            } else {
                break;
            }
        } else {
            println!("empty list");
            break;
        }
    }

    println!("slow={:?} ", slow.as_ref().map(|n| { n.val }));
    println!("fast={:?} ", fast.as_ref().map(|n| { n.val }));
    // while let Some(n) = half {
    //     println!("n.val={}", n.val);
    //     half = n.next();
    // }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::assert_eq;
    #[test]
    fn test_reorder_list() {
        let n = ListNode::new(1);
    }
}
