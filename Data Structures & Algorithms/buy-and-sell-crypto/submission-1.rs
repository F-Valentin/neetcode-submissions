impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.is_empty() {
            return 0;
        }

        let mut bigger_profit = 0;

        for i in 0..prices.len() {
            for j in (i + 1)..prices.len() {
                if prices[i] < prices[j] {
                    let profit = prices[j] - prices[i];

                    if  profit > bigger_profit {
                        bigger_profit = profit;
                    } 
                } 
            }
        }

        bigger_profit
    }
}
