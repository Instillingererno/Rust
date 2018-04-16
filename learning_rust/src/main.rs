#![feature(type_ascription)]

extern crate rand;

use std::ops::Add;
use rand::Rng;

struct Die {
    value: u8,
}
impl Die {
    fn new() -> Die {
        Die { value : rand::thread_rng().gen_range(1,7), }
    }
}
impl Add for Die {
    type Output = u8;

    fn add(self, other: Die) -> u8 {
        self.value + other.value
    }
}

struct Hand {
    dice : [Die; 5],
}
impl Hand {
    fn new() -> Hand {
        let hand = [Die::new(), Die::new(), Die::new(), Die::new(), Die::new()];
        Hand {
            dice : hand,
        }
    }
    fn print(self) -> String {
        let mut output: String = "".to_string();
        for a in self.dice.iter() {
            output.add(&a.value.to_string());
        };
        return output.to_string();
    }
}

fn main() {
    let die1 = Die::new();
    let die2 = Die::new();
    println!("Die1's value is {}", die1.value);
    println!("Die2's value is {}", die2.value);
    println!("The total is {}", die1.add(die2));
}
