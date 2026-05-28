impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.is_empty() {
            return 0;
        }

        let mut bigger_profit = 0;
        let mut min_price = prices[0];

        for price in prices {
            if price < min_price {
                min_price = price;
                continue;
            }
            bigger_profit = bigger_profit.max(price - min_price);
        }

        bigger_profit
    }
}
