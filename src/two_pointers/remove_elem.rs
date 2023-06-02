#[allow(dead_code)]
pub fn remove_elemement(nums: &mut Vec<i32>, val: i32) -> i32 {
    // [3, 2, 3]
    // [3, 2, 2, 3]
    // [0, 1, 2, 2, 0, 4, 2]
    println!("val={val}");
    match nums.is_empty() {
        true => 0,
        false => {
            let mut beg_i = 0;
            let mut end_i = nums.len();
            let mut count = 0;
            loop {
                println!("beg_i={beg_i} end_i={end_i} count={count} {:?}", nums);
                if beg_i == end_i {
                    break;
                }
                if nums[beg_i] == val {
                    for i in (beg_i..end_i).rev() {
                        println!("  {i}: {}", nums[i]);
                        if nums[i] != val {
                            println!("  find nums[{i}]={}", nums[i]);
                            nums[beg_i] = nums[i];
                            nums[i] = val;
                            count += 1;
                            println!("  {:?}", nums);
                            end_i = i;
                            break;
                        }
                    }
                } else {
                    count += 1;
                }
                beg_i += 1;
                // if count == 10 {
                //     break;
                // }
            }
            count
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_eq;

    #[allow(unused_mut, unused_variables)]
    #[test]
    fn test_remove_elemement() {
        let empty_vec: Vec<i32> = Vec::new();
        let mut vec: Vec<i32> = Vec::new();
        let res = remove_elemement(&mut vec, 2);
        assert_eq!(0, res);
        assert_eq!(empty_vec, vec);

        let mut vec: Vec<i32> = vec![2];
        let res = remove_elemement(&mut vec, 2);
        assert_eq!(0, res);
        assert_eq!(vec![2], vec);
        let mut temp_vec = vec.split_off(0);
        assert_eq!(empty_vec, vec);

        let mut vec: Vec<i32> = vec![3];
        let res = remove_elemement(&mut vec, 2);
        assert_eq!(1, res);
        assert_eq!(vec![3], vec);

        let mut vec: Vec<i32> = vec![3, 2, 2, 3];
        let res = remove_elemement(&mut vec, 2);
        let mut temp_vec = vec.split_off(2);
        vec.sort();
        assert_eq!(2, res);
        assert_eq!(vec![3, 3], vec);

        let mut vec: Vec<i32> = vec![0, 1, 3, 2, 2, 0, 4, 2];
        let res = remove_elemement(&mut vec, 2);
        let mut temp_vec = vec.split_off(5);
        vec.sort();
        assert_eq!(5, res);
        assert_eq!(vec![0, 0, 1, 3, 4], vec);
    }
}
