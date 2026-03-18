use clap::Parser;
use std::process::ExitCode;
use uuid::Uuid;

#[derive(Parser)]
#[command(author, version, about = "FriendlyID tool — converts UUIDs to URL-friendly Base62 IDs and vice versa")]
struct Cli {
    /// UUID to encode, or FriendlyID to decode. Omit to generate a new FriendlyID
    #[arg(value_name = "UUID|FRIENDLYID")]
    id: Option<String>,
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match cli.id {
        Some(id) => match convert(&id) {
            Ok(result) => {
                println!("{result}");
                ExitCode::SUCCESS
            }
            Err(e) => {
                eprintln!("{e}");
                ExitCode::FAILURE
            }
        },
        None => {
            println!("{}", friendly_id::create());
            ExitCode::SUCCESS
        }
    }
}

/// Detects format by presence of `-` (UUIDs always contain hyphens, Base62 never does)
fn convert(id: &str) -> Result<String, String> {
    if id.contains('-') {
        Uuid::parse_str(id)
            .map(|uuid| friendly_id::encode(&uuid))
            .map_err(|e| format!("Invalid UUID '{id}': {e}"))
    } else {
        friendly_id::decode(id)
            .map(|uuid| uuid.to_string())
            .map_err(|e| format!("Invalid FriendlyID '{id}': {e}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode() {
        assert_eq!(
            convert("5wbwf6yUxVBcr48AMbz9cb").unwrap(),
            "c3587ec5-0976-497f-8374-61e0c2ea3da5"
        );
    }

    #[test]
    fn test_encode() {
        assert_eq!(
            convert("c3587ec5-0976-497f-8374-61e0c2ea3da5").unwrap(),
            "5wbwf6yUxVBcr48AMbz9cb"
        );
    }

    #[test]
    fn test_create() {
        let id = friendly_id::create();
        assert!(!id.is_empty());
        assert!(friendly_id::decode(&id).is_ok(), "created ID '{id}' should be decodeable");
    }

    #[test]
    fn test_roundtrip() {
        let uuid = "c3587ec5-0976-497f-8374-61e0c2ea3da5";
        let encoded = convert(uuid).unwrap();
        let decoded = convert(&encoded).unwrap();
        assert_eq!(decoded, uuid);
    }

    #[test]
    fn test_uppercase_uuid() {
        assert_eq!(
            convert("C3587EC5-0976-497F-8374-61E0C2EA3DA5").unwrap(),
            "5wbwf6yUxVBcr48AMbz9cb"
        );
    }

    #[test]
    fn test_invalid_uuid() {
        assert!(convert("not-a-uuid").is_err());
    }

    #[test]
    fn test_invalid_friendly_id() {
        assert!(convert("!!!invalid").is_err());
    }
}
