#[allow(dead_code)]
pub fn max_lower_or_equal(v: &Vec<i32>, x: i32) -> i32 {
    let mut l: usize = 0;
    let mut r: usize = v.len();
    println!("{:?} v.len()={} v0={}", v, v.len(), v.get(0).unwrap());
    if v.len() == 0 || *v.get(0).unwrap() > x {
        return -1;
    }
    println!("go!");
    while l + 1 < r {
        let mid = (l + r) / 2;
        println!("mid={mid} {l}-{r}");
        if *v.get(mid).unwrap() <= x {
            println!("  {} <= {x}", v.get(mid).unwrap());
            l = mid;
        } else {
            println!("  {} > {x}", v.get(mid).unwrap());
            r = mid;
        }
    }
    l as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_eq;
    #[test]
    fn test_mloe() {
        assert_eq!(max_lower_or_equal(&vec![5, 6], 1), -1);
        assert_eq!(max_lower_or_equal(&vec![0, 2, 5, 5, 6], 5), 3);
        assert_eq!(max_lower_or_equal(&vec![0, 2, 5, 6, 6], 6), 4);
        assert_eq!(max_lower_or_equal(&vec![0], 6), 0);
    }
}
