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
    //
    // // let high: u64 = (i >> 64) as u64;
    // // let low: u64 = i as u64;
    // let mostSignificantBits: u64 = i.hi();
    // let leastSignificantBits: u64 = i.lo();
    // let jeden: u128 = 1;
    //
    // println!("xxxx... = {} - {}", leastSignificantBits, mostSignificantBits);
    // println!("xx... = {} - {}", jeden, (jeden << 64));
    // let i1 = (jeden << 64) as u64;
    // let ttt = (i1.mul(  mostSignificantBits));
    // let x = ttt.add(leastSignificantBits);
    // println!("ca... = {} ", x);


    println!("bigint... = {}", my_uuid.as_u128().to_biguint().unwrap());
    println!("uuid... = {}", encode(&my_uuid));


}

fn encode(uuid: &Uuid) -> String {
    let data = uuid.as_u128();
    let base62 = base62(data);
    return base62.to_string();
}


fn decode(id: &String) -> Uuid {
    // return Uuid::parse_str(id).unwrap();
    // let decoded: BigInteger = Base62::decode(&id);
    base_62::decode(&id);
    return Uuid::parse_str(id).unwrap();
    // return UuidConverter::to_uuid(decoded);
}

fn base62(number: u128) -> String {
    let alphabet = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    let mut n = number;
    let basis = 62;

    let mut ret = String::from("");

    while n > u128::MIN {
        let temp = ( n % basis) as usize;
        let x = alphabet.chars().nth(temp).unwrap();
        // ret = concat!(x.to_string(), ret).to_string();
        ret = [x.to_string(), ret].concat();
        n = n / basis;
    }
    return ret.to_string();
}


