//! Given an integer array nums and an integer val, remove all occurrences of
//! val in nums in-place. The order of the elements may be changed. Then return
//! the number of elements in nums which are not equal to val.
//!
//! Consider the number of elements in nums which are not equal to val be k, to
//! get accepted, you need to do the following things:
//!
//! Change the array nums such that the first k elements of nums contain the
//! elements which are not equal to val. The remaining elements of nums are not
//! important as well as the size of nums.
//!
//! Return k.

pub struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut p = 0;

        for i in 0..nums.len() {
            if nums[i] != val {
                nums[p] = nums[i];
                p += 1;
            }
        }
        p as i32
    }
}

#[test]
fn test_a() {
    let mut nums = vec![3, 2, 2, 3];
    let val = 3;
    let output = Solution::remove_element(&mut nums, val);

    assert_eq!(output, 2);

    let mut a = nums[..2].to_vec();
    a.sort_unstable();

    let b = vec![2, 2];

    assert_eq!(a, b);
}

#[test]
fn test_b() {
    let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
    let val = 2;
    let output = Solution::remove_element(&mut nums, val);

    assert_eq!(output, 5);

    let mut a = nums[..5].to_vec();
    a.sort_unstable();

    let b = vec![0, 0, 1, 3, 4];

    assert_eq!(a, b);
}
