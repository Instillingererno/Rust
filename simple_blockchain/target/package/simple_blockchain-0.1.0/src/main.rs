#![feature(plugin, custom_derive)]
#![plugin(serde_macros)]

extern crate time;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate sha2;


#[macro_use]
mod blockchain;

fn main() {
    println!("Hello, world!");
}
