// My first Rust program as an attempt at: https://www.hackerrank.com/challenges/sherlock-and-the-beast 
extern crate time;

use std::io;
use time::PreciseTime;

// fn main() {
//     for digit_count in Tests::new().skip(1) {
//         match generate_key(digit_count) {
//             Key::None => println!("-1"),
//             Key::DecentNumber(k) => println!("{}", k),
//         }
//     }
// }

fn main() {
    let start = PreciseTime::now();
    for digit_count in 1..100000 {
        generate_key(digit_count);
    }
    let end = PreciseTime::now();
    println!("{} seconds", start.to(end));
}


pub fn generate_key(digits: u32) -> Key {
    if digits < 3 {
        return Key::None;
    } else {
        match find_decent_number(digits) {
            DecentNumber::None => Key::None,
            DecentNumber::Number{fives, threes} => if fives > 0 && threes > 0 {
                return Key::DecentNumber(repeat(Number::Five, fives as usize) + &repeat(Number::Three, threes as usize));
            } else if fives > 0 {
                return Key::DecentNumber(repeat(Number::Five, fives as usize));
            } else {
                Key::DecentNumber(repeat(Number::Three, threes as usize))
            },
        }
    }
}

pub fn find_decent_number(digits: u32) -> DecentNumber {
    if digits.decent_fives() {
        DecentNumber::Number{fives: digits, threes: 0}
    } else if digits < 3 {
        DecentNumber::None
    } else {
        let mut fives = digits;
        let mut threes = 0;

        while fives % 3 != 0 {
            fives -= 1;
            threes += 1;
            if fives.decent_fives() && threes.decent_threes() {
                return DecentNumber::Number { fives: fives, threes: threes };
            }
        }    

        while fives != 0 {
            fives -= 3;
            threes += 3;
            if fives.decent_fives() && threes.decent_threes() {
                return DecentNumber::Number { fives: fives, threes: threes };
            }
        }

        DecentNumber::None
    }
}

pub fn repeat(num: Number, count: usize) -> String {
    let number = match num {
        Number::Five => "5",
        Number::Three => "3",
    };
        
    std::iter::repeat(number).take(count).collect::<String>()
}


struct Tests {
    complete: bool,
}

impl Tests {
    fn new() -> Tests {
        Tests {
            complete: false,
        }
    }
}

impl Iterator for Tests {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        if !self.complete {
            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(len) => if len > 0 { 
                        return Option::Some(input.trim().parse::<u32>().unwrap());
                  } else {
                        self.complete = true;
                        return Option::None;
                  },
                Err(error) => panic!("this is a terrible mistake {}", error),
            };
        } else {
            return Option::None;
        }
    }
}

trait IsDecentNumber {
    fn decent_fives(&self) -> bool;
    fn decent_threes(&self) -> bool;
}

impl IsDecentNumber for u32 {
    fn decent_fives(&self) -> bool {
        self % 3 == 0
    }
    
    fn decent_threes(&self) -> bool {
        self % 5 == 0
    }
}

pub enum Key {
    None,
    DecentNumber(String),
}

pub enum Number {
    Five,
    Three,
}

pub enum DecentNumber {
    None,
    Number{fives: u32, threes: u32},
}
