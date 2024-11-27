mod retirement;
fn main() {
    // ("1995-12", "原法定退休年龄50周岁女职工", "2050-12,55,60"),
    // ("1971-04", "原法定退休年龄55周岁女职工", "2026-08,55.33,4"),
    // let res = retirement::retire_time("1995-12", "原法定退休年龄50周岁女职工");
    let res = retirement::retire_time("1971-04", "原法定退休年龄55周岁女职工");
    println!("{res}");
}
