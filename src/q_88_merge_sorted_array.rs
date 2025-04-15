//! You are given two integer arrays nums1 and nums2, sorted in non-decreasing
//! order, and two integers m and n, representing the number of elements in
//! nums1 and nums2 respectively.
//!
//! Merge nums1 and nums2 into a single array sorted in non-decreasing order.
//!
//! The final sorted array should not be returned by the function, but instead
//! be stored inside the array nums1. To accommodate this, nums1 has a length
//! of m + n, where the first m elements denote the elements that should be
//! merged, and the last n elements are set to 0 and should be ignored. nums2
//! has a length of n.
//!
//! Example 1:
//!
//! Input: nums1 = [1,2,3,0,0,0], m = 3, nums2 = [2,5,6], n = 3
//! Output: [1,2,2,3,5,6]
//! Explanation: The arrays we are merging are [1,2,3] and [2,5,6].
//! The result of the merge is [1,2,2,3,5,6] with the underlined elements
//! coming from nums1.
//!
//! Example 2:
//!
//! Input: nums1 = [1], m = 1, nums2 = [], n = 0
//! Output: [1]
//! Explanation: The arrays we are merging are [1] and [].
//! The result of the merge is [1].
//! Example 3:
//!
//! Input: nums1 = [0], m = 0, nums2 = [1], n = 1
//! Output: [1]
//! Explanation: The arrays we are merging are [] and [1].
//! The result of the merge is [1].
//! Note that because m = 0, there are no elements in nums1. The 0 is only there
//! to ensure the merge result can fit in nums1.
pub struct Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        // If nums2 is empty, no need to merge
        if n == 0 {
            return;
        }

        // If nums1 is empty, copy nums2 to nums1
        if m == 0 {
            nums1.copy_from_slice(nums2);
            return;
        }

        // We need to merge
        let mut i = m as usize - 1;
        let mut j = n as usize - 1;

        for k in (0..nums1.len()).rev() {
            if nums1[i] > nums2[j] {
                nums1[k] = nums1[i];

                // If nums1 is empty, copy the rest of nums2 to nums1. Note
                // the inclusion of j in the slice
                if i == 0 {
                    nums1[..k].copy_from_slice(&nums2[..=j]);
                    return;
                }
                i -= 1;
            } else {
                nums1[k] = nums2[j];

                // If nums2 is empty, no need to merge anymore
                if j == 0 {
                    return;
                }
                j -= 1;
            }
        }
    }

    pub fn merge2(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut x = m - 1;
        let mut y = n - 1;
        let mut z = m + n - 1;

        while y >= 0 {
            if x >= 0 && nums1[x as usize] > nums2[y as usize] {
                nums1[z as usize] = nums1[x as usize];
                x -= 1;
            } else {
                nums1[z as usize] = nums2[y as usize];
                y -= 1;
            }
            z -= 1;
        }
    }
}

#[test]
fn test_a() {
    let mut nums1 = vec![1, 2, 3, 0, 0, 0];
    let mut nums2 = vec![2, 5, 6];
    let m = 3;
    let n = 3;
    let expected = vec![1, 2, 2, 3, 5, 6];
    Solution::merge(&mut nums1, m, &mut nums2, n);
    assert_eq!(expected, nums1);
}

#[test]
fn test_b() {
    let mut nums1 = vec![1];
    let mut nums2 = vec![];

    let m = 1;
    let n = 0;
    let expected = vec![1];
    Solution::merge(&mut nums1, m, &mut nums2, n);
    assert_eq!(expected, nums1);
}

#[test]
fn test_c() {
    let mut nums1 = vec![0];
    let mut nums2 = vec![1];

    let m = 0;
    let n = 1;
    let expected = vec![1];
    Solution::merge(&mut nums1, m, &mut nums2, n);
    assert_eq!(expected, nums1);
}

#[test]
fn test_d() {
    let mut nums1 = vec![2, 0];
    let mut nums2 = vec![1];

    let m = 1;
    let n = 1;
    let expected = vec![1, 2];
    Solution::merge(&mut nums1, m, &mut nums2, n);
    assert_eq!(expected, nums1);
}
