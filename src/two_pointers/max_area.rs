#[allow(dead_code)]
pub fn max_area(height: Vec<i32>) -> i32 {
    // println!("{:?}", height);

    let mut cur_i1 = 0;
    let mut cur_i2 = height.len() - 1;
    let mut max_area = 0;

    while cur_i1 < cur_i2 {
        let l = height.get(cur_i1).unwrap();
        let r = height.get(cur_i2).unwrap();
        let cur_area = if l > r {
            (cur_i2 - cur_i1) as i32 * r
        } else {
            (cur_i2 - cur_i1) as i32 * l
        };
        // println!(
        //     "cur_i1={cur_i1} cur_i2={cur_i2} l={l} r={r} cur_area={cur_area} max_area={max_area}"
        // );
        if cur_area > max_area {
            max_area = cur_area;
        }
        if l < r {
            cur_i1 += 1;
        } else {
            cur_i2 -= 1;
        }
    }
    max_area
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_eq;
    #[test]
    fn test_max_area() {
        assert_eq!(max_area(vec![1, 1]), 1);
        assert_eq!(max_area(vec![1, 1, 1, 8, 9]), 8);
        assert_eq!(max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    }
}
