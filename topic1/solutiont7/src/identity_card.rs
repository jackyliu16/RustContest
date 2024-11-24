use super::gb11643_1989::GB11643_1989;

pub fn check_id_card(id_card_no: &str) -> String {
    let gb = id_card_no.parse::<GB11643_1989>();
    println!("{}", gb.unwrap());
    String::new()
}

pub trait IdentityCard {
    fn get_region() -> &'static str;
    fn get_birth_date() -> &'static str;
    fn get_order() -> &'static str;
    fn check() -> bool;
}
