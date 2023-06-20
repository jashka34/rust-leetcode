#[allow(dead_code)]
pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    println!("\n{:?} target={target}", nums);
    if nums.len() == 0 {
        return -1;
    }
    let (mut l, mut r) = (0, nums.len());
    while l < r {
        println!("{l}-{r}");
        let mid = l + ((r - l) >> 1);
        println!("  mid={mid}");
        println!("    {}", nums[mid]);
        if nums[mid] == target {
            return mid as i32;
        } else if target > nums[mid] {
            l = mid + 1;
        } else {
            r = mid;
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_eq;
    #[test]
    fn test_search() {
        assert_eq!(search(vec![1, 3, 5, 6], 5), 2);
        assert_eq!(search(vec![-1, 3, 5, 6], 5), 2);
        assert_eq!(search(vec![-1, 3, 5, 6], -1), 0);
        assert_eq!(search(vec![-1, 3, 5, 7, 9], 9), 4);
        assert_eq!(search(vec![], 2), -1);
        assert_eq!(search(vec![-1, 0, 3, 5, 9, 12], 2), -1);
    }
}
