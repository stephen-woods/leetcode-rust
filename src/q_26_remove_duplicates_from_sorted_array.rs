//! Given an integer array nums sorted in non-decreasing order, remove the
//! duplicates in-place such that each unique element appears only once. The
//! relative order of the elements should be kept the same. Then return the
//! number of unique elements in nums.
//!
//! Consider the number of unique elements of nums to be k, to get accepted,
//! you need to do the following things:
//!
//! Change the array nums such that the first k elements of nums contain the
//! unique elements in the order they were present in nums initially. The
//! remaining elements of nums are not important as well as the size of nums.
//!
//! Return k.
//! Custom Judge:
//!
//! The judge will test your solution with the following code:
//!
//! int[] nums = [...]; // Input array
//! int[] expectedNums = [...]; // The expected answer with correct length
//!
//! int k = removeDuplicates(nums); // Calls your implementation
//!
//! assert k == expectedNums.length;
//! for (int i = 0; i < k; i++) {
//!     assert nums[i] == expectedNums[i];
//! }
//! If all assertions pass, then your solution will be accepted.

pub struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut last = nums[0];
        let mut p = 1;

        for i in 1..nums.len() {
            if nums[i] != last {
                nums[p] = nums[i];
                last = nums[i];
                p += 1;
            }
        }
        p as i32
    }
}

#[test]
fn test_a() {
    let mut nums = vec![1, 1, 2];
    let output = Solution::remove_duplicates(&mut nums);
    assert_eq!(output, 2);

    let mut a = nums[..2].to_vec();
    a.sort_unstable();

    let b = vec![1, 2];
    assert_eq!(a, b);
}

#[test]
fn test_b() {
    let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    let output = Solution::remove_duplicates(&mut nums);
    assert_eq!(output, 5);

    let mut a = nums[..5].to_vec();
    a.sort_unstable();

    let b = vec![0, 1, 2, 3, 4];

    assert_eq!(a, b);
}
