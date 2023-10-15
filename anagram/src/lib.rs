pub fn anagram_string(s: String, t: String) {
    anagraming::is_anagram_hashmap(s,t);
}
mod anagraming{
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
        let mut map = std::collections::HashMap::new();
        s.chars().for_each(|c| *map.entry(c).or_insert(0) += 1); 
        t.chars().for_each(|c| *map.entry(c).or_insert(0) -= 1); 
        !map.into_values().any(|count| count != 0)
     }
}