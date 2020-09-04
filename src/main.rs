use clap::{App, Arg};
use uuid::Uuid;

fn main() {
    let matches = App::new("FriendlyId App")
        .version("0.1.0")
        .author("Mariusz Smykula <mariuszs@gmail.com>")
        .about("FriendlyId converter")
        .arg(Arg::with_name("ID")
            .about("ID to convert")
            .required(true)
            .index(1))
        .arg(Arg::with_name("decode")
            .short('d')
            .long("decode")
            .takes_value(false)
            .about("Decode friendlyId"))
        .get_matches();

    let id = matches.value_of("ID").expect("Missing id!");

    if matches.is_present("decode") {
        println!("{}", decode(id.to_string()));
    } else {
        if id.contains("-") {
            let my_uuid = Uuid::parse_str(id).expect("Invalid uuid format!");
            println!("{}", encode(&my_uuid));
        } else {
            println!("{}", decode(id.to_string()));
        }
    }
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
    let decode_base62 = decode_base_62(id.to_string());
    return Uuid::from_u128(decode_base62);
}


fn decode_base_62(string: String) -> u128 {
    let alphabet = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    let base = 62;
    let strlen = string.len() as u128;
    let mut num = 0 as u128;
    let mut idx = 0 as u128;

    for char in string.chars() {
        let power = (strlen - (idx + 1)) as u32;
        let i = u128::pow(base, power) as u128;
        num = alphabet.find(char).unwrap() as u128 * i + num;
        idx = idx + 1;
    }

    return num;
}



