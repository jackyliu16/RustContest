mod simple2traditional;

fn main() {
    let input = "发财";
    let tp = "s2t";
    let res = simple2traditional::converter(input, tp);
    println!("res: {res}");
}
