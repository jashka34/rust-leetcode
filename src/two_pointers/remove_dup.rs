#[allow(dead_code)]
pub fn remove_duplicates1(nums: &mut Vec<i32>) -> i32 {
    if nums.len() == 0 {
        return 0;
    }
    let mut uniq_count = 1;
    let mut ret_nums: Vec<i32> = Vec::new();
    ret_nums.push(*nums.get(0).unwrap());
    for (i, val) in nums.iter().enumerate() {
        if i > 0 {
            let prev_val = nums.iter().nth(i - 1).unwrap();
            // println!("({i}) val={val} prev_val={prev_val} nums={:?}", nums);
            if prev_val != val {
                uniq_count += 1;
                ret_nums.push(*val)
            }
        }
    }
    nums.clear();
    for rn in ret_nums.iter() {
        nums.push(*rn);
    }
    println!("nums={:?}", nums);
    uniq_count
}

#[allow(dead_code)]
pub fn remove_duplicates2(nums: &mut Vec<i32>) -> i32 {
    match nums.is_empty() {
        true => 0,
        false => {
            let mut prev_i = 0;
            for i in 1..nums.len() {
                // println!("prev_i({prev_i})={}, i({i})={}", nums[prev_i], nums[i]);
                if nums[i] != nums[prev_i] {
                    prev_i += 1;
                    nums[prev_i] = nums[i];
                }
            }
            (prev_i + 1) as i32
        }
    }
}

#[allow(dead_code)]
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    remove_duplicates2(nums)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_eq;

    #[allow(unused_mut, unused_variables)]
    #[test]
    fn test_remove_duplicates() {
        let empty_vec: Vec<i32> = Vec::new();
        let mut vec0: Vec<i32> = Vec::new();
        let res0 = remove_duplicates(&mut vec0);
        assert_eq!(0, res0);
        assert_eq!(empty_vec, vec0);
        let mut vec1: Vec<i32> = vec![1, 2, 2];
        let res1 = remove_duplicates(&mut vec1);
        let mut temp_vec = vec1.split_off(2);
        assert_eq!(2, res1);
        assert_eq!(vec![1, 2], vec1);
        let mut vec2: Vec<i32> = vec![1, 2, 2, 4, 5, 8];
        let res2 = remove_duplicates(&mut vec2);
        let mut temp_vec = vec2.split_off(5);
        assert_eq!(vec![1, 2, 4, 5, 8], vec2);
        assert_eq!(5, res2);
        let mut vec3: Vec<i32> = vec![1, 2];
        let res3 = remove_duplicates(&mut vec3);
        let mut temp_vec = vec3.split_off(2);
        assert_eq!(2, res3);
        assert_eq!(vec![1, 2], vec3);
        let mut vec4: Vec<i32> = vec![1];
        let res4 = remove_duplicates(&mut vec4);
        let mut temp_vec = vec4.split_off(1);
        assert_eq!(1, res4);
        assert_eq!(vec![1], vec4);
        let mut vec5: Vec<i32> = vec![1, 2, 2, 4, 4, 4, 4, 4, 4, 4, 5, 8];
        let res5 = remove_duplicates(&mut vec5);
        let mut temp_vec = vec5.split_off(5);
        assert_eq!(vec![1, 2, 4, 5, 8], vec5);
        assert_eq!(5, res5);
    }
}
