use uuid::Uuid;

fn main() {
    let my_uuid = Uuid::parse_str("c3587ec5-0976-497f-8374-61e0c2ea3da5").unwrap();

    println!("uuid... = {}", encode(&my_uuid));
    println!("dec... = {}", decode_base_62("5wbwf6yUxVBcr48AMbz9cb".to_string()));
    println!("dec... = {}", decode("5wbwf6yUxVBcr48AMbz9cb".to_string()));
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



