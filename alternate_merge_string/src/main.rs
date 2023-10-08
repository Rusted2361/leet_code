
impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut merged = String::new();
        let mut chars1 = word1.chars();
        let mut chars2 = word2.chars();

        while let (Some(c1), Some(c2)) = (chars1.next(), chars2.next()) {
            merged.push(c1);
            merged.push(c2);
        }
        merged
    }
}

fn main() {
    println!("Hello, world!");
}
