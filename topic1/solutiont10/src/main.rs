use aint::u31;

mod zuc_encryption;
mod copy;

fn main() {
    let inp = String::from("特朗普");
    let result = zuc_encryption::encryption(inp);
    println!("{result}");

    // let inp = "特朗普";
    // let encryption = copy::encryption((*inp).to_string());
    // println!("{}", encryption);
}
// dbg!(u31!(0x0000_FFFF)); // 65535
// dbg!(u31!(0x7FFF_0000)); // 2147418112
//
// println!(" ================================================ ");
//
// dbg!(u31!(0x7FFF_FFFF).overflowing_shl(15).0); // 2147450880
// dbg!(u31!(0x7FFF_FFFF).overflowing_shl(16).0); // 2147418112
// dbg!(u31!(0x7FFF_FFFF).overflowing_shl(17).0); // 2147352576
//
// println!(" ================================================ ");
//
// dbg!(u31!(0x7FFF_FFFF).overflowing_shr(15).0); // 65535
// dbg!(u31!(0x7FFF_FFFF).overflowing_shr(16).0); // 32767
// dbg!(u31!(0x7FFF_FFFF).overflowing_shr(17).0); // 16383
//
// println!(" ================================================ ");
