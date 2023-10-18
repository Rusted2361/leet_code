    pub fn repeated_substring_pattern(s: String) -> bool {
        let len = s.len();
        for i in 1..len {
             if len % i == 0 {
                let repeated = &s[..i];
                if repeated.repeat(len / i) == s {
                    return true;
                }
            }
        }
        false
    }

fn main() {
    println!("Hello, world!");
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repeated_substring_pattern_true() {
        assert_eq!(Solution::repeated_substring_pattern("abcabc".to_string()), true);
        assert_eq!(Solution::repeated_substring_pattern("abab".to_string()), true);
        // Add more test cases for patterns that should return true
    }

    #[test]
    fn test_repeated_substring_pattern_false() {
        assert_eq!(Solution::repeated_substring_pattern("abc".to_string()), false);
        assert_eq!(Solution::repeated_substring_pattern("aabb".to_string()), false);
        // Add more test cases for patterns that should return false
    }
}
