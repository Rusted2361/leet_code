fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }
    let mut s_chars: Vec<char> = s.chars().collect();
    let mut t_chars: Vec<char> = t.chars().collect();

    // Sort the character arrays
    s_chars.sort();
    t_chars.sort();

    s_chars == t_chars
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anagrams() {
        assert_eq!(is_anagram("listen".to_string(), "silent".to_string()), true);
        assert_eq!(is_anagram("hello".to_string(), "world".to_string()), false);
        assert_eq!(is_anagram("abcdef".to_string(), "abcdefg".to_string()), false);
    }
}

fn main() {
    println!("Hello, world!");
}