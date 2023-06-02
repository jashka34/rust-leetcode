#[allow(dead_code)]
fn is_pv1(s: String) -> bool {
    println!("1) {s}");
    let mut s2: String = String::new();
    for ch in s.chars() {
        if ch.is_alphanumeric() {
            s2.push(ch.to_lowercase().next().unwrap());
        }
    }
    let s = s2;
    println!("2) {s}");
    for i in 0..(s.len() / 2) {
        let ch1 = s.chars().nth(i).unwrap();
        let i2 = s.len() - i - 1;
        let ch2 = s.chars().nth(i2).unwrap();
        println!("{i} ch1={ch1} {i2} ch2={ch2}");
        if ch1 != ch2 {
            return false;
        }
        // println!("{i} {i2}");
    }
    true
}

#[allow(dead_code)]
fn is_pv2(s: String) -> bool {
    let mut cur_i1 = 0;
    let mut cur_i2 = s.len() - 1;
    let bs = s.as_bytes();
    // println!("{s} {cur_i1} {cur_i2}");
    while cur_i1 < cur_i2 {
        let ch1 = bs.get(cur_i1).unwrap();
        let ch2 = bs.get(cur_i2).unwrap();
        if !ch1.is_ascii_alphanumeric() {
            cur_i1 += 1;
            continue;
        }
        if !ch2.is_ascii_alphanumeric() {
            cur_i2 -= 1;
            continue;
        }
        if ch1.to_ascii_lowercase() != ch2.to_ascii_lowercase() {
            return false;
        } else {
            cur_i1 += 1;
            cur_i2 -= 1;
        }
    }
    true
}

#[allow(dead_code)]
pub fn is_palindrome(s: String) -> bool {
    is_pv2(s)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_eq;

    #[test]
    fn test_palindrome() {
        assert!(is_palindrome(" ".to_string()));
        assert!(is_palindrome("a".to_string()));
        assert!(is_palindrome("aBba".to_string()));
        assert!(is_palindrome("aBcba".to_string()));
        assert!(!is_palindrome("aBcbad".to_string()));
        assert!(is_palindrome("# a !bc&ba ".to_string()));
        assert!(is_palindrome("A man, a plan, a canal: Panama".to_string()));
    }
}
