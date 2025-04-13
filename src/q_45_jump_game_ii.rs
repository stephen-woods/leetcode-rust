//! You are given a 0-indexed array of integers nums of length n. You are
//! initially positioned at nums[0].
//!
//! Each element nums[i] represents the maximum length of a forward jump from
//! index i. In other words, if you are at nums[i], you can jump to any
//! nums[i + j] where:
//!
//! 0 <= j <= nums[i] and
//! i + j < n
//! Return the minimum number of jumps to reach nums[n - 1]. The test cases are
//! generated such that you can reach nums[n - 1].
//!
//!  
//!
//! Example 1:
//!
//! Input: nums = [2,3,1,1,4]
//! Output: 2
//! Explanation: The minimum number of jumps to reach the last index is 2.
//! Jump 1 step from index 0 to 1, then 3 steps to the last index.
//!
//! Example 2:
//!
//! Input: nums = [2,3,0,1,4]
//! Output: 2
//!  
//!
//! Constraints:
//!
//! 1 <= nums.length <= 104
//! 0 <= nums[i] <= 1000
//! It's guaranteed that you can reach nums[n - 1].

pub struct Solution;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        // The number of jumps needed to reach the last index
        let mut jumps = 0;

        // The maximum index that can be reached within the current jump
        let mut max_reachable = 0_usize;

        // The maximum index that was reached from the previous jump
        let mut current_end = 0_usize;

        // We are only concerned with all the indexes except the last one
        let nums_except_last = &nums[..nums.len() - 1];

        for (i, &v) in nums_except_last.iter().enumerate() {
            max_reachable = max_reachable.max(i + v as usize);

            if i == current_end {
                jumps += 1;
                current_end = max_reachable;
            }
        }

        jumps
    }
}

#[test]
fn test_a() {
    let nums = vec![2, 3, 1, 1, 4];
    let output = Solution::jump(nums);

    assert_eq!(output, 2);
}

#[test]
fn test_b() {
    let nums = vec![2, 3, 0, 1, 4];
    let output = Solution::jump(nums);

    assert_eq!(output, 2);
}
