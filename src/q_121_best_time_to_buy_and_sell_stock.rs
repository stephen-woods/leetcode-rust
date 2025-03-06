//! You are given an array prices where prices[i] is the price of a given stock
//! on the ith day.
//!
//! You want to maximize your profit by choosing a single day to buy one stock and
//! choosing a different day in the future to sell that stock.
//!
//! Return the maximum profit you can achieve from this transaction. If you cannot
//! achieve any profit, return 0.
//!
//!  
//!
//! Example 1:
//!
//! Input: prices = [7,1,5,3,6,4]
//! Output: 5
//! Explanation: Buy on day 2 (price = 1) and sell on day 5 (price = 6), profit = 6-1 = 5.
//! Note that buying on day 2 and selling on day 1 is not allowed because you must buy before you sell.
//! Example 2:
//!
//! Input: prices = [7,6,4,3,1]
//! Output: 0
//! Explanation: In this case, no transactions are done and the max profit = 0.
//!  
//!
//! Constraints:
//!
//! 1 <= prices.length <= 105
//! 0 <= prices[i] <= 104

use std::cmp::{max, min};
pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() == 1 {
            return 0;
        }

        let mut floor = prices[0];
        let mut max_profit = 0;
        for &i in prices.iter().skip(1) {
            let profit = i - floor;
            max_profit = max(profit, max_profit);
            floor = min(i, floor);
        }
        max_profit
    }
}

#[test]
fn test_a() {
    let prices = vec![7, 1, 5, 3, 6, 4];
    let result = Solution::max_profit(prices);
    assert_eq!(result, 5);
}

#[test]
fn test_b() {
    let prices = vec![7, 6, 4, 3, 1];
    let result = Solution::max_profit(prices);
    assert_eq!(result, 0);
}
