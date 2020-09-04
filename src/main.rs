use std::convert::TryFrom;
use std::convert::TryInto;
// use num_traits::{Zero, One};
use std::mem::replace;
use std::ops::{Add, Mul};

use num_bigint::{BigUint, ToBigUint};
use uuid::Uuid;

mod UuidConverter;
mod Url62;

fn main() {
    println!("Hello, world!");

    let input = vec![0xDE, 0xAD, 0xBE, 0xEF];
    let encoded = base_62::encode(&input);
    println!("0xDEADBEEF = {}", encoded);
    let deadbeef = base_62::decode("JsoUl8").unwrap();

    let input = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt";
    let encoded = base_62::encode(input.as_bytes());
    println!("lorem... = {}", encoded);
    let loremipsum = base_62::decode("Inj62xrWzFT5RgFoP72ZkfbrMabXdyZeYGijtTt8zuBN4XvHvEw6x2pk2BtdepGle57axcSeY2ixeXqOvwpE2VaEE3pHeeumHvIbZf0qUUxRBg99NrIALFCE").unwrap();
    let greeting = "Hello there.".to_string();

    let my_uuid = Uuid::parse_str("c3587ec5-0976-497f-8374-61e0c2ea3da5").unwrap();

    let i = my_uuid.as_u128();
    println!("xxxx... = {}", i);
    // println!("bigint... = {}", my_uuid.as_bytes().);


    println!("bigint... = {}", my_uuid.as_u128().to_biguint().unwrap());
    println!("uuid... = {}", encode(&my_uuid));
    println!("dec... = {}", decodeBase62("5wbwf6yUxVBcr48AMbz9cb".to_string()));
    println!("dec... = {}", decode("5wbwf6yUxVBcr48AMbz9cb".to_string()));
    // println!("dec... = {}", decodeBase62("A".to_string()));
    // println!("dec... = {}", decodeBase62("3844".to_string()));
}

fn encode(uuid: &Uuid) -> String {
    let data = uuid.as_u128();
    let base62 = base62(data);
    return base62.to_string();
}


fn base62(number: u128) -> String {
    let alphabet = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    let mut n = number;
    let basis = 62;

    let mut ret = String::from("");

    while n > 0 {
        let temp = (n % basis) as usize;
        let x = alphabet.chars().nth(temp).unwrap();
        ret = [x.to_string(), ret].concat();
        n = n / basis;
    }
    return ret.to_string();
}

fn decode(id: String) -> Uuid {
    let decode_base62 = decodeBase62(id.to_string());
    return Uuid::from_u128(decode_base62);
}


fn decodeBase62(string: String) -> u128 {
    let alphabet = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    let base = 62;
    let strlen = string.len() as u128;
    println!("strlen... = {}", strlen);
    let mut num = 0 as u128;
    let mut idx = 0 as u128;

    for char in string.chars() {
        println!("char = '{}'", char);
        let power = (strlen - (idx + 1)) as u32 ;
        println!("power... = {}", power);
        let i = u128::pow(base, power) as u128;
        println!("i... = {}", i);
        // let i = base.pow(power);
        // let i1 = alphabet.chars().position(|c| c == char).unwrap() as u128;
        // let i1 = alphabet.find(char).unwrap() as u128;
        // println!("i1 = {}", i1);
         num = alphabet.find(char).unwrap() as u128 * i + num;
        println!("num = {}", num);
         idx = idx + 1;
        println!("idx = {}", idx);
    }

    return num;
}



