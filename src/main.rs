use clap::{App, Arg, crate_authors, crate_version};
use friendly_id;
use uuid::Uuid;

fn main() {
    let matches = App::new("FriendlyId Converter")
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .about("The FriendlyID library converts a given UUID to a URL-friendly ID which is based on Base62")
        .arg(Arg::with_name("ID")
            .help("UUID to encode or ID to decode")
            .required(false)
            .index(1))
        .get_matches();
    let id = match matches.value_of("ID") {
        Some(id) => String::from(   convert(id)),
        None => create(),
    };
    println!("{}", id);
}

fn create() -> String {
    friendly_id::create()
}

fn convert(id: &str) -> String {
    if id.contains("-") {
        match Uuid::parse_str(&id) {
            Ok(uuid) => return friendly_id::encode(&uuid),
            Err(error) => panic!("Invalid uuid '{}': {:?}", id, error)
        };
    } else {
        match friendly_id::decode(id) {
            Ok(uuid) => return uuid.to_string(),
            Err(error) => panic!("Invalid id '{}': {:?}", id, error)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{convert, create};

    #[test]
    fn test_decode() {
        assert_eq!(convert("5wbwf6yUxVBcr48AMbz9cb"),
                   "c3587ec5-0976-497f-8374-61e0c2ea3da5");
    }

    #[test]
    fn test_encode() {
        assert_eq!(convert("c3587ec5-0976-497f-8374-61e0c2ea3da5"),
                   "5wbwf6yUxVBcr48AMbz9cb");
    }

    #[test]
    fn test_create() {
        assert!(!create().is_empty());
    }
}





