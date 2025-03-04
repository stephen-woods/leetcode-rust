//! Given an array nums of size n, return the majority element.
//!
//! The majority element is the element that appears more than ⌊n / 2⌋ times. You
//! may assume that the majority element always exists in the array.
//!
//!
//! Example 1:
//!
//! Input: nums = [3,2,3]
//! Output: 3
//! Example 2:
//!
//! Input: nums = [2,2,1,1,1,2,2]
//! Output: 2
//!  
//!
//! Constraints:
//!
//! n == nums.length
//! 1 <= n <= 5 * 104
//! -109 <= nums[i] <= 109
//!  
//!
//! Follow-up: Could you solve the problem in linear time and in O(1) space?

pub struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut v = nums[0];
        let mut c = 1;

        for &x in nums.iter().skip(1) {
            if c == 0 {
                v = x;
                c += 1;
            } else if x == v {
                c += 1;
            } else {
                c -= 1;
            }
        }
        v
    }
}

#[test]
fn test_a() {
    let nums = vec![3, 2, 3];
    let result = Solution::majority_element(nums);

    assert_eq!(result, 3);
}

#[test]
fn test_b() {
    let nums = vec![2, 2, 1, 1, 1, 2, 2];
    let result = Solution::majority_element(nums);

    assert_eq!(result, 2);
}
