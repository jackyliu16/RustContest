use crate::identity_card::check_id_card;

mod identity_card;

fn main() {
    // let id_no = "370725 881105 149";
    let id_no = "370725881105149";
    let res = check_id_card(id_no);
    println!("{res}");
}
