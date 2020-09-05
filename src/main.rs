use clap::{crate_version, App, Arg};
use uuid::Uuid;

mod friendly_id;
mod base62;

fn main() {
    let matches = App::new("FriendlyId Converter")
        .version(crate_version!())
        .author("Mariusz Smykula <mariuszs@gmail.com>")
        .about("The FriendlyID library converts a given UUID to a URL-friendly ID which is based on Base62")
        .arg(Arg::with_name("ID")
            .help("UUID to convert or ID to decode")
            .required(true)
            .index(1))
        .arg(Arg::with_name("decode")
            .short("d")
            .long("decode")
            .takes_value(false)
            .help("Decode friendlyId"))
        .get_matches();

    let id = matches.value_of("ID").expect("Missing id!");

    if matches.is_present("decode") {
        println!("{}", friendly_id::decode(id.to_string()));
    } else {
        if id.contains("-") {
            let my_uuid = Uuid::parse_str(id).expect("Invalid uuid format!");
            println!("{}", friendly_id::encode(&my_uuid));
        } else {
            println!("{}", friendly_id::decode(id.to_string()));
        }
    }
}







