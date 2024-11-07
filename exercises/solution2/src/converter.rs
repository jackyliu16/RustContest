pub fn convert_base(num_str: &str, to_base: u32) -> String {
    let parts: Vec<&str> = num_str.split('(').collect();        // OPTIMIZE: could use regax
    let (var, radix): (&str, u32) = (
        parts[0],
        parts[1].trim_end_matches(')').parse::<u32>().unwrap(), // FIXME: may out of border
    );

    if radix < 2 || radix > 16 || to_base < 2 || to_base > 16 || var.len() == 0 {
        return String::new();                                   // FIXME: Should break with panic or error msg
    }

    match convert(var, radix, to_base) {
        Ok(ok) => ok.into_iter().collect(),
        Err(_) => String::from("0"),
    }
}

fn convert(
    num_str: &str,
    from_base: u32,
    to_base: u32,
) -> Result<Vec<char>, std::num::ParseIntError> {
    dbg!("{}, {}, {}", num_str, from_base, to_base);
    let digits = "0123456789abcdef";
    let mut res: Vec<char> = Vec::new();

    // let mut num = num_str.parse::<i64>()?;                   // i64::from_str_radix ?
    let mut num = u128::from_str_radix(num_str, from_base)?;     // num_str.parse::<i64>() will failure in lower
    while num > 0 {
        let digit: usize = (num % to_base as u128) as usize;
        res.push(digits.chars().nth(digit).unwrap());
        num /= to_base as u128;
    }

    if res.len() == 0 {
        res.push('0');
    }

    Ok(res.into_iter().rev().collect())
}
