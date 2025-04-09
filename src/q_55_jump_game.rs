//! You are given an integer array nums. You are initially positioned at the
//! array's first index, and each element in the array represents your maximum
//! jump length at that position.
//!
//! Return true if you can reach the last index, or false otherwise.
//!
//!  
//!
//! Example 1:
//!
//! Input: nums = [2,3,1,1,4]
//! Output: true
//! Explanation: Jump 1 step from index 0 to 1, then 3 steps to the last index.
//! Example 2:
//!
//! Input: nums = [3,2,1,0,4]
//! Output: false
//! Explanation: You will always arrive at index 3 no matter what. Its maximum
//! jump length is 0, which makes it impossible to reach the last index.
//!  
//!
//! Constraints:
//!
//! 1 <= nums.length <= 104
//! 0 <= nums[i] <= 105

pub struct Solution;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut far = nums[0];

        let nums_len = nums.len() as i32;

        for (i, num) in nums.iter().enumerate().skip(1) {
            if i as i32 > far {
                return false;
            }
            let candidate = i as i32 + num;
            if candidate > nums_len {
                return true;
            }

            far = far.max(candidate);
        }
        true
    }
}

#[test]
fn test_a() {
    let nums = vec![2, 3, 1, 1, 4];
    let output = Solution::can_jump(nums);

    assert!(output);
}

#[test]
fn test_b() {
    let nums = vec![3, 2, 1, 0, 4];
    let output = Solution::can_jump(nums);

    assert!(!output);
}

#[test]
fn test_c() {
    let nums = vec![0];
    let output = Solution::can_jump(nums);

    assert!(output);
}
#[test]
fn test_d() {
    let nums = vec![0, 1];
    let output = Solution::can_jump(nums);

    assert!(!output);
}
