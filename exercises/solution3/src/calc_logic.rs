pub fn new_birthday_probability(n: u32) -> f64 {
    if n > 365 {
        return 1.0_f64;
    }

    let mut poss = 1.0_f64;                     // 1 - P(没有人在同一天过生日)

    for i in 0..n {
        poss *= (365 - i) as f64 / 365.0_f64;       // FIXME: four-digit decimation
    }

    1.0_f64 - poss
}
