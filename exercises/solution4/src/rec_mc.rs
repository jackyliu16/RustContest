pub fn dp_rec_mc(amount: u32) -> u32 {
    let coin_price: [usize; 8] = [1, 2, 5, 10, 20, 30, 50, 100];
    let mut dp = vec![std::usize::MAX; amount as usize + 1]; // Currently optimal number of money
    dp[0] = 0;

    // dp[i] = min(dp[i], dp[i - coin[j]] + 1)
    for i in 0..=amount as usize {
        for j in coin_price.iter().rev().filter(|&&x| x <= amount as usize && x <= i) {
            dp[i as usize] = std::cmp::min(dp[i], dp[i - j] + 1);
        }
    }

    dp[amount as usize] as u32
}
