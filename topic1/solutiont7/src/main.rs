mod error;
mod gb11643_1999;
mod identity_card;
mod gb11643_1989;

fn main() {
    // let id_no = "370725 881105 149";
    let id_no = "370725881105149";
    let res = identity_card::check_id_card(id_no);
    println!("{res}");
}
