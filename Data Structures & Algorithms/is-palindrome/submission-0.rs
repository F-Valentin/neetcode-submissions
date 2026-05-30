impl Solution {
    pub fn is_palindrome(mut s: String) -> bool {
        s.retain(|c| c.is_alphanumeric());
        let s_rev: String = s.clone().chars().rev().collect();

        s.to_lowercase() == s_rev.to_lowercase()
    }
}
