#[allow(dead_code)]
pub fn trap(height: Vec<i32>) -> i32 {
    if height.len() <= 2 {
        return 0;
    }
    let mut res = 0;
    let mut l_i = 0;
    let mut r_i = height.len() - 1;
    let mut l_h = height.get(l_i).unwrap();
    let mut r_h = height.get(r_i).unwrap();
    let mut l_max = l_h;
    let mut r_max = r_h;
    while l_i < r_i {
        // println!("l_i={l_i} r_i={r_i} l_h={l_h} r_h={r_h} l_max={l_max} r_max={r_max} res={res}");
        if l_h < r_h {
            l_i += 1;
            l_h = height.get(l_i).unwrap();
            if l_h < l_max {
                res += l_max - l_h;
            } else {
                l_max = l_h;
            }
        } else {
            r_i -= 1;
            r_h = height.get(r_i).unwrap();
            if r_h < r_max {
                res += r_max - r_h;
            } else {
                r_max = r_h;
            }
        }
    }
    res
}

#[cfg(test)]
mod test {
    use std::assert_eq;

    use super::*;
    #[test]
    fn test_trap() {
        assert_eq!(trap(vec![3]), 0);
        assert_eq!(trap(vec![3, 1]), 0);
        assert_eq!(trap(vec![3, 1, 3]), 2);
        assert_eq!(trap(vec![3, 1, 2, 1, 3]), 5);
        assert_eq!(trap(vec![3, 1, 2, 1, 2]), 2);
        assert_eq!(trap(vec![0, 3, 1, 2, 1, 2, 0]), 2);
        assert_eq!(trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
    }
}
