#[allow(dead_code)]
pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    // println!("\n{:?} target={target}", nums);
    let mut l = 0;
    let mut r = nums.len();
    let mut cnt = 0;
    while l < r {
        let mid = (l + r) / 2;
        // let cur = *nums.get(mid).unwrap();
        let cur = nums[mid];
        // println!("{l}-{r} mid={mid} cur={cur}");
        if cur == target {
            return mid as i32;
        } else if cur < target {
            l = mid + 1;
            // println!("  now l={l}");
        } else {
            r = mid;
            // println!("  now r={r}");
        }
        cnt += 1;
        // println!("  cnt={cnt}");
        if cnt > nums.len() {
            break;
        }
    }
    // println!("ret={l}");
    l as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_eq;
    #[test]
    fn test_si() {
        assert_eq!(search_insert(vec![1, 3, 5, 6], 5), 2);
        assert_eq!(search_insert(vec![1, 3, 5, 6], 2), 1);
        assert_eq!(search_insert(vec![1, 3, 5, 6], 1), 0);
        assert_eq!(search_insert(vec![1, 3, 5, 6], 7), 4);
        assert_eq!(search_insert(vec![1, 3, 5, 6], 9), 4);
        assert_eq!(search_insert(vec![1, 3, 5, 6], 6), 3);
        assert_eq!(search_insert(vec![1, 3, 5, 6, 8], 7), 4);
        assert_eq!(search_insert(vec![1, 3, 5, 6, 8], 3), 1);
        assert_eq!(search_insert(vec![], 2), 0);
    }
}
