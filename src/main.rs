#![no_std]
#![no_main]

use defmt::export::display;

#[embassy::main]
async fn main() {
    display("Hello, world!");
}
