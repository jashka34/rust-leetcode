#[allow(unused_imports)]
use binary_search::sqrt::*;
mod binary_search;
use binary_search::search_insert_position::*;

// use crate::two_pointers::*;
// use std::println;
// pub mod two_pointers;
// mod two_pointers;
// use two_pointers::reorder_list::*;

fn main() {
    // let vec = vec![0, 2, 3, 5, 5, 8, 8, 9];

    // let r = max_lower_or_equal(&vec, 5);
    //
    let r = my_sqrt(11);
    println!("r={r}");
    let r = my_sqrt(12);
    println!("r={r}");
    let r = my_sqrt(13);
    println!("r={r}");
    // let mut n1 = ListNode::new(1);
    // let mut n2 = ListNode::new(2);
    // let mut n3 = ListNode::new(3);
    // let mut n4 = ListNode::new(4);
    // let n5 = ListNode::new(5);
    //
    // n4.next = Some(Box::new(n5));
    // n3.next = Some(Box::new(n4));
    // n2.next = Some(Box::new(n3));
    // n1.next = Some(Box::new(n2));
    //
    // reorder_list(&mut Some(Box::new(n1)));
    //
    // let mut n1 = ListNode::new(1);
    // let mut n2 = ListNode::new(2);
    // let mut n3 = ListNode::new(3);
    // let mut n4 = ListNode::new(4);
    // let mut n5 = ListNode::new(5);
    // let n6 = ListNode::new(6);
    //
    // n5.next = Some(Box::new(n6));
    // n4.next = Some(Box::new(n5));
    // n3.next = Some(Box::new(n4));
    // n2.next = Some(Box::new(n3));
    // n1.next = Some(Box::new(n2));
    //
    // reorder_list(&mut Some(Box::new(n1)));
    // let res = str_str("abcd".to_string(), "bc".to_string());
    // println!("res={res}");

    // let mut vec = vec![0, 3, 1, 2, 2, 0, 4, 2];
    // let mut vec = vec![6, 2, 3, 2, 2, 1, 0, 3];
    // let mut vec = vec![3, 2, 2, 3];
    // let mut vec: Vec<i32> = vec![0, 1, 2, 2, 0, 4, 2];
    // let r = remove_duplicates(&mut vec);
    // println!("{:?}", vec);
    // let r = remove_elemement(&mut vec, 2);
    // println!("r={r} vec={:?}", vec);
    // let vec2 = vec.split_off(r as usize);
    // vec.sort();
    // println!("vec={:?} vec2={:?}", vec, vec2);
}
