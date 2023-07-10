// macro_rules! nvec {
//     ( $($x:expr );* ) => {
//         $($x:expr),*
//         // vec![
//         //     $(
//         //            vec![$($x),* ],
//         //       )*
//         // ]
//     };
// }

#[allow(dead_code)]
pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    println!("\n{:?} -> {target}", matrix);
    // let mut cnt = 0;
    let x_len = matrix[0].len();
    let y_len = matrix.len();
    let (mut l, mut r) = (0, (x_len * y_len) - 1);
    while l <= r {
        let mid = l + ((r - l) >> 1);
        let x = mid % x_len;
        let y = mid / x_len;
        println!("  {l}-{r} mid={mid} x={x} y={y}");
        let cur = matrix[y][x];
        println!("    cur={cur} target={target}");
        if cur == target {
            return true;
        } else if l == r {
            return false;
        } else {
            if cur < target {
                l = mid + 1;
            } else {
                r = mid;
            }
        }
        // cnt += 1;
        // if cnt > 10 {
        //     break;
        // }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_eq;
    #[test]
    fn test_sqrt() {
        // #![feature(trace_macros)]
        // trace_macros!(true);
        // let v1 = nvec![
        //     1, 4, 5;
        //     6, 9, 10;
        //     11, 14, 16
        // ];
        // trace_macros!(false);
        // println!("v1={:?}", v1);
        assert_eq!(
            search_matrix(
                vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 63]],
                3
            ),
            true
        );
        assert_eq!(
            search_matrix(
                vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 63]],
                30
            ),
            true
        );
        assert_eq!(
            search_matrix(
                vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 63]],
                63
            ),
            true
        );
        assert_eq!(
            search_matrix(
                vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 63]],
                1
            ),
            true
        );
        assert_eq!(
            search_matrix(
                vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 63]],
                13
            ),
            false
        );
    }
}
