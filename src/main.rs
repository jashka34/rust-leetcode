#[allow(unused_imports)]
use binary_search::search_matrix::*;
mod binary_search;

// use crate::two_pointers::*;
// use std::println;
// pub mod two_pointers;
// mod two_pointers;
// use two_pointers::reorder_list::*;

fn main() {
    // let vec = vec![0, 2, 3, 5, 5, 8, 8, 9];

    let r = search_matrix(
        vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 63]],
        3,
    );
    println!("r1={r}");
    let r = search_matrix(
        vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 63]],
        30,
    );
    println!("r2={r}");
    let r = search_matrix(
        vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 63]],
        63,
    );
    println!("r3={r}");
    let r = search_matrix(
        vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 63]],
        13,
    );
    println!("r3={r}");
    // let r = max_lower_or_equal(&vec, 5);
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
