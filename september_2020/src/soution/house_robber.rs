impl crate::soution::Solution {
    #[allow(dead_code)]
    ///Dynamic-programming
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        if nums.len() == 1 {
            return nums[0];
        }
        let mut dp: Vec<i32> = vec![];
        dp[0] = nums[0];
        dp[1] = std::cmp::max(nums[0], nums[1]);
        for i in 2..nums.len() {
            dp[i] = std::cmp::max(dp[i - 2] + nums[i], dp[i - 1]);
        }
        return dp[nums.len() - 1];
    }
}
