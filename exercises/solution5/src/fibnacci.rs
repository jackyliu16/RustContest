pub fn odd_fibnacci_sum(threshold: u32) -> u32 {
    // 其实 fibnacci sum 的减半就相当于结果（在最后一位为偶数的情况）
    let mut prev: u32 = 1;
    let mut curr: u32 = 1;
    let mut res = 1; // first one
    let mut flag: bool = true;

    // Something like sliding window, only contains the contents calculating.
    while curr < threshold {
        if curr % 2 != 0 {
            res += curr;
        }
        curr += prev;
        prev = curr - prev;
    }

    res
}
