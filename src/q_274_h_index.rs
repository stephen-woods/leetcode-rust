//! Given an array of integers citations where citations[i] is the number of
//! citations a researcher received for their ith paper, return the
//! researcher's h-index.
//!
//! According to the definition of h-index on Wikipedia: The h-index is
//! defined as the maximum value of h such that the given researcher has
//! published at least h papers that have each been cited at least h times.
//!
//!  
//!
//! Example 1:
//!
//! Input: citations = [3,0,6,1,5]
//! Output: 3
//! Explanation: [3,0,6,1,5] means the researcher has 5 papers in total and
//! each of them had received 3, 0, 6, 1, 5 citations respectively. Since
//! the researcher has 3 papers with at least 3 citations each and the
//! remaining two with no more than 3 citations each, their h-index is 3.
//!
//! Example 2:
//!
//! Input: citations = [1,3,1]
//! Output: 1
//!  
//!
//! Constraints:
//!
//! n == citations.length
//! 1 <= n <= 5000
//! 0 <= citations[i] <= 1000

pub struct Solution;

impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        for h in (1..=citations.len()).rev() {
            let hh = h as i32;
            let mut count = 0;
            for c in &citations {
                if *c >= hh {
                    count += 1;
                }
            }
            if count >= hh {
                return hh;
            }
        }
        0
    }
}

#[test]
pub fn test_a() {
    let input = Vec::from([4, 0, 6, 1, 5]);
    let result = Solution::h_index(input);

    assert_eq!(result, 3);
}

#[test]
pub fn test_b() {
    let input = Vec::from([1, 3, 1]);
    let result = Solution::h_index(input);

    assert_eq!(result, 1);
}
