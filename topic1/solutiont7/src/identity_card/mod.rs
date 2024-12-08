//! 参照 GB 11643-1989 和 GB 11643-1999 实现的身份证号码正确性检验算法
use std::fmt::Display;
use std::str::FromStr;
use gb11643_1989::GB11643_1989;
use gb11643_1999::GB11643_1999;

mod common;
mod error;
mod gb11643_1989;
mod gb11643_1999;

/// 检测输入的 id_card_no 是否符合对应的输入规范
pub fn check_id_card(id_card_no: &str) -> String {
    match id_card_no.len() {
        15 => {
            if let Ok(gb) = id_card_no.parse::<GB11643_1989>() {
                format!("身份证号码正确,{}", gb.to_string())
            } else {
           String::from("身份证号码错误")
        }
        },
        _ => {
            if let Ok(gb) = id_card_no.parse::<GB11643_1999>() {
                format!("身份证号码正确,{}", gb.to_string())
            } else {
                String::from("身份证号码错误")
            }
        }
    }
}
pub trait GB11643: FromStr + Display { }