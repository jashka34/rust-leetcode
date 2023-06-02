#[allow(dead_code)]
pub fn str_str(haystack: String, needle: String) -> i32 {
    // abcdef bc
    // println!("{haystack} {needle}");
    let mut res: i32 = -1;
    match needle.len() == 0 {
        true => {
            res = -1;
            res
        }
        false => {
            for hi in 0..haystack.len() {
                // println!("hi={hi} hc={}", haystack.chars().nth(hi).unwrap());
                match (haystack.len() - hi) < needle.len() {
                    true => {
                        break;
                    }
                    false => {
                        let mut ok = true;
                        for (ni, nc) in needle.chars().enumerate() {
                            let hi2 = hi + ni;
                            let hc2 = haystack.chars().nth(hi2).unwrap();
                            // println!("  ni={ni} nc={nc} hc2={hc2}");
                            if hc2 != nc {
                                ok = false;
                                break;
                            }
                        }
                        // println!("  !!! hi={hi} ok={ok}");
                        if ok {
                            res = hi as i32;
                            break;
                        }
                    }
                };
            }
            // println!("res={res} v1");
            res
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_eq;

    #[test]
    fn test_str_str() {
        assert_eq!(str_str("abcdef".to_string(), "bc".to_string()), 1);
        assert_eq!(str_str("abcdef".to_string(), "abc".to_string()), 0);
        assert_eq!(str_str("abcdbef".to_string(), "bc".to_string()), 1);
        assert_eq!(str_str("abcdbcef".to_string(), "bcf".to_string()), -1);
        assert_eq!(str_str("".to_string(), "bc".to_string()), -1);
        assert_eq!(str_str("b".to_string(), "bc".to_string()), -1);
        assert_eq!(str_str("bc".to_string(), "bc".to_string()), 0);
        assert_eq!(str_str("b".to_string(), "b".to_string()), 0);
        assert_eq!(str_str("abc".to_string(), "".to_string()), -1);
        assert_eq!(str_str("".to_string(), "".to_string()), -1);
        assert_eq!(str_str("mississippi".to_string(), "issip".to_string()), 4);
    }
}
