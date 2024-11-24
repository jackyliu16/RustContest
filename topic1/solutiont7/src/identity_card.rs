use std::fmt::Display;
use std::str::FromStr;
use super::gb11643_1989::GB11643_1989;
use super::gb11643_1999::GB11643_1999;
use serde_json::to_string;
use crate::error::{ParseError};

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