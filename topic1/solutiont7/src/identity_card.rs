use std::fmt::Display;
use std::str::FromStr;

pub fn check_id_card(id_card_no: &str) -> String {
    String::new()
}

pub trait GB11643: FromStr + Display {}