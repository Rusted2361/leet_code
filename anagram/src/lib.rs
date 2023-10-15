pub fn anagram_string(s: String, t: String) {
    anagraming::is_anagram_hashmap(s,t);
}
mod anagraming{
    use std::collections::HashMap;
    pub fn is_anagram_sort(s: String, t: String) -> bool {
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
    pub fn is_anagram_hashmap(s: String, t: String) -> bool {
        let mut map = HashMap::new();
        s.chars().for_each(|c| *map.entry(c).or_insert(0) += 1); 
        t.chars().for_each(|c| *map.entry(c).or_insert(0) -= 1); 
        !map.into_values().any(|count| count != 0)
     }
     fn count_chars(s: String) -> HashMap::<char, i32> {
        s.chars().fold(HashMap::<char, i32>::new(), |mut map, ch| {
            map.entry(ch).and_modify(|count| *count += 1 ).or_insert(1);
            map
        })
    }
    
    pub fn is_anagram_fold(s: String, t: String) -> bool {
            count_chars(s) == count_chars(t)
        }
    
}