use clap::Parser;
use uuid::Uuid;

#[derive(Parser)]
#[command(author, version, about = "The FriendlyID library converts a given UUID to a URL-friendly ID which is based on Base62")]
struct Cli {
    /// UUID to encode or ID to decode
    id: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    let id = match cli.id {
        Some(id) => convert(&id),
        None => create(),
    };
    println!("{id}");
}

fn create() -> String {
    friendly_id::create()
}

fn convert(id: &str) -> String {
    if id.contains('-') {
        match Uuid::parse_str(id) {
            Ok(uuid) => friendly_id::encode(&uuid),
            Err(error) => {
                eprintln!("Invalid uuid '{id}': {error:?}");
                std::process::exit(1);
            }
        }
    } else {
        match friendly_id::decode(id) {
            Ok(uuid) => uuid.to_string(),
            Err(error) => {
                eprintln!("Invalid id '{id}': {error:?}");
                std::process::exit(1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{convert, create};

    #[test]
    fn test_decode() {
        assert_eq!(
            convert("5wbwf6yUxVBcr48AMbz9cb"),
            "c3587ec5-0976-497f-8374-61e0c2ea3da5"
        );
    }

    #[test]
    fn test_encode() {
        assert_eq!(
            convert("c3587ec5-0976-497f-8374-61e0c2ea3da5"),
            "5wbwf6yUxVBcr48AMbz9cb"
        );
    }

    #[test]
    fn test_create() {
        assert!(!create().is_empty());
    }
}
