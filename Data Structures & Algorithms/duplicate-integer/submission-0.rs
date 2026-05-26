impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        if nums.is_empty() || nums.len() == 1 {
            return false;
        }

        let mut nums_sorted = nums.clone();

        nums_sorted.sort();

        for i in 0..nums_sorted.len() - 1 {
            if nums_sorted[i] == nums_sorted[i + 1] {
                return true;
            }
        }

        return false;
    }
}
