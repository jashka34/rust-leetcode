#[allow(dead_code)]
fn get_next_dif_char_idx(s: &str, cur_char_idx: usize) -> usize {
    let mut cur_idx: usize = cur_char_idx;
    // println!(
    //     "  cur_char={cur_char} cur_char_idx={cur_char_idx} cur_idx={cur_idx} s.len()={}",
    //     s.len()
    // );
    while cur_idx < s.len()
        && s.chars().nth(cur_char_idx).unwrap() == s.chars().nth(cur_idx).unwrap()
    {
        // println!("  > --- cur_idx={cur_idx}");
        cur_idx += 1;
    }
    cur_idx
}

#[allow(dead_code)]
pub fn get_max_consecutive_elements(s: &str) -> usize {
    let mut result: usize = 0;
    let mut cur_char_idx: usize = 0;
    // println!("{s}");
    while cur_char_idx < s.len() {
        // println!("> --- cur_char_idx={cur_char_idx}");
        let next_dif_char_idx = get_next_dif_char_idx(s, cur_char_idx);
        if (next_dif_char_idx - cur_char_idx) > result {
            result = next_dif_char_idx - cur_char_idx;
        }
        // println!("< --- next_dif_char_idx={next_dif_char_idx} result={result}");
        cur_char_idx = next_dif_char_idx;
    }
    result
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_eq;
    #[test]
    fn test_max_consecutive() {
        assert_eq!(2, get_max_consecutive_elements("assa"));
        assert_eq!(3, get_max_consecutive_elements("absssa"));
        assert_eq!(3, get_max_consecutive_elements("absss"));
        assert_eq!(2, get_max_consecutive_elements("aas"));
        assert_eq!(2, get_max_consecutive_elements("aa"));
        assert_eq!(1, get_max_consecutive_elements("a"));
        assert_eq!(0, get_max_consecutive_elements(""));
        assert_eq!(4, get_max_consecutive_elements("aaaabsssrr"));
        assert_eq!(3, get_max_consecutive_elements("aabsssrr"));
        assert_eq!(5, get_max_consecutive_elements("aabsssrrrrr"));
    }
}
