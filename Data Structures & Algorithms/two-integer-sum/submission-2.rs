impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.is_empty() {
            return Vec::new();
        }

        let mut indexes: Vec<i32> = Vec::new();

        for i in 0..nums.len() - 1 {
            for j in (i + 1)..nums.len() {
                if nums[i] + nums[j] == target {
                    indexes.push(i as i32);
                    indexes.push(j as i32);
                }
            }
        }

        if indexes.len() == 2 {
            indexes        
        } else {
            Vec::new()
        }
    }
}
