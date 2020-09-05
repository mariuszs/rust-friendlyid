use clap::{App, Arg, crate_authors, crate_version};
use friendly_id;
use uuid::Uuid;

fn main() {
    let matches = App::new("FriendlyId Converter")
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .about("The FriendlyID library converts a given UUID to a URL-friendly ID which is based on Base62")
        .arg(Arg::with_name("ID")
            .help("UUID to convert or ID to decode")
            .required(false)
            .index(1))
        .arg(Arg::with_name("decode")
            .short("d")
            .long("decode")
            .takes_value(false)
            .help("Decode friendlyId"))
        .get_matches();

    match matches.value_of("ID") {
        None => println!("{}", friendly_id::create()),
        Some(id) => {
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
    }
}







