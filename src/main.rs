extern crate ark;
extern crate term_painter;

use term_painter::{ToStyle, Color};
use term_painter::Color::*;

use ark::manager;
use ark::models;


fn main() {

    println!("{}",
    BrightMagenta.bold().paint("
 ########   ##     ##   ######   ########        ###     ########   ##    ##
 ##     ##  ##     ##  ##    ##     ##          ## ##    ##     ##  ##   ##
 ##     ##  ##     ##  ##           ##         ##   ##   ##     ##  ##  ##
 ########   ##     ##   ######      ##        ##     ##  ########   #####
 ##   ##    ##     ##        ##     ##        #########  ##   ##    ##  ##
 ##    ##   ##     ##  ##    ##     ##        ##     ##  ##    ##   ##   ##
 ##     ##   #######    ######      ##        ##     ##  ##     ##  ##    ##")
        );

    println!("");
    println!("{}", Cyan.bold().paint("Fetching newests posts from ArkCoin..."));
    println!("");

    let manager = manager::Manager::default();

    let t = manager.get_received_transactions("AZHXnQAYajd3XkxwwiL6jnLjtDHjtAATtR");

    match t {
    Ok(response) => print_transaction(response.transactions),
    Err(e) => println!("Error: {:?}", e),
}

fn print_transaction(transactions: Option<Vec<models::Transaction>>) {
    if let Some(mut unwrapped_transactions) = transactions {
        for entry in unwrapped_transactions.iter_mut() {
            if let Some(ref vendor_field) = entry.vendorField {
                println!("{}", Green.bold().paint(vendor_field));
                println!("");
            }
        }
    }
}
}
