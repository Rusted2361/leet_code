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
