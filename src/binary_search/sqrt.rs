#[allow(dead_code)]
pub fn my_sqrt(x: i32) -> i32 {
    println!("\nx={x} sqrt(x)={}", (x as f64).sqrt());
    if x == 0 {
        return 0 as i32;
    }
    let (mut l, mut r) = (1, x);
    // let mut c = 0;
    while l <= r {
        // c += 1;
        let mid = l + ((r - l) >> 1);
        // let mid2 = mid * mid;
        let mid2 = x / mid;
        println!("  {l}-{r} mid={mid} mid2={mid2}");
        if mid == mid2 {
            return mid;
        } else if mid < mid2 {
            l = mid + 1;
        } else {
            r = mid - 1;
        }
    }
    r
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_eq;
    #[test]
    fn test_sqrt() {
        assert_eq!(my_sqrt(11), 3);
        assert_eq!(my_sqrt(4), 2);
        assert_eq!(my_sqrt(8), 2);
        assert_eq!(my_sqrt(9), 3);
        assert_eq!(my_sqrt(2147395599), 46339);
        assert_eq!(my_sqrt(0), 0);
        assert_eq!(my_sqrt(1), 1);
    }
}
