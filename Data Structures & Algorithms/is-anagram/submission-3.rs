impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.is_empty() || t.is_empty() || s.len() != t.len() {
            return false;
        } 

        let mut s_vec: Vec<char> = s.chars().collect();
        let mut t_vec: Vec<char> = t.chars().collect();

        s_vec.sort();
        t_vec.sort();

        s_vec == t_vec
    }
}
