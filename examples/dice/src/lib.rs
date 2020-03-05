#![no_std]
extern crate libw;
use libw::*;

#[no_mangle]
pub fn _start() {
    println("welcome to dice roller ( type 'help' for commands )");
    let text = read_line();
}
