use std::str::FromStr;

use clap::{crate_authors, crate_description, crate_version, Parser};

// TODO: rot13 & rotN
// TODO: base32 & base64 with different alphabets
#[derive(Debug, PartialEq, Eq)]
pub enum Encoding {
    Raw,
    ASCII,
    UTF8,
    Base32,
    Base64,
    Hex,
}

impl ToString for Encoding {
    fn to_string(&self) -> String {
        match self {
            Self::Raw => "raw".into(),
            Self::ASCII => "ascii".into(),
            Self::UTF8 => "utf8".into(),
            Self::Base32 => "base32".into(),
            Self::Base64 => "base64".into(),
            Self::Hex => "hex".into(),
        }
    }
}

impl FromStr for Encoding {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "raw" => Ok(Self::Raw),
            "ascii" => Ok(Self::ASCII),
            "utf8" => Ok(Self::UTF8),
            "hex" => Ok(Self::Hex),
            "base32" => Ok(Self::Base32),
            "base64" => Ok(Self::Base64),
            _ => Err(format!("Unknown encoding: {}", s)),
        }
    }
}

#[derive(Debug, Parser)]
pub enum OutputFormat {
    Raw,
    Safe,
}

impl FromStr for OutputFormat {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "raw" => Ok(Self::Raw),
            "safe" => Ok(Self::Safe),
            _ => Err(format!("Unknown output format: {}", s)),
        }
    }
}

#[derive(Debug, Parser)]
#[clap(version = crate_version!(), author = crate_authors!(), after_help = crate_description!())]
pub struct Args {
    /// Tip: provide this as a quick shortcut!
    #[clap(name = "type", number_of_values = 2, multiple_occurrences = false)]
    pub encoding: Vec<Encoding>,

    /// The file to read from or `-` to read from STDIN
    #[clap(short = 'i', long = "in", default_value = "-")]
    pub input: String,

    /// The encoding of the input: "raw", "ascii", "utf8", "base32", "base64" or "hex"
    #[clap(
        short = 'I',
        long = "in-type",
        required_unless_present = "type",
        conflicts_with = "type"
    )]
    pub input_type: Option<Encoding>,

    /// The file to write to or `-` to write to STDOUT
    #[clap(short = 'o', long = "out", default_value = "-")]
    pub output: String,

    /// The encoding of the output: "raw", "ascii", "utf8", "base32", "base64" or "hex"
    #[clap(
        short = 'O',
        long = "out-type",
        required_unless_present = "type",
        conflicts_with = "type"
    )]
    pub output_type: Option<Encoding>,

    /// Controls how data is printed to STDOUT: "raw" (like `cat`) or "safe"
    #[clap(short = 'F', long = "out-format")]
    pub output_format: Option<OutputFormat>,
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::cli::Encoding;

    #[test]
    fn serde() {
        let t = |e: Encoding| assert_eq!(e, Encoding::from_str(&e.to_string()).unwrap());
        t(Encoding::Raw);
        t(Encoding::ASCII);
        t(Encoding::UTF8);
        t(Encoding::Base32);
        t(Encoding::Base64);
        t(Encoding::Hex);
    }
}
