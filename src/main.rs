extern crate fnv;
extern crate nx;

mod item_provider;
mod my_node;

use item_provider::*;

fn main() {
    let pro = ItemProvider::new();
    println!("{:?}", pro.get_item_data(2190000));
    println!("{:?}", pro.get_item_data(2190001));
}
