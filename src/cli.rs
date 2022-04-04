use std::str::FromStr;

use clap::{crate_authors, crate_description, crate_version, ArgEnum, Parser};

// TODO: rot13 & rotN
// TODO: binary, octal
#[derive(Debug, ArgEnum, Clone, PartialEq, Eq)]
pub enum Encoding {
    Raw,
    // Character encodings
    ASCII,
    UTF8,
    // Digit encodings
    Hex,
    // Base32
    #[clap(name("base32"))]
    Base32Rfc4648,
    #[clap(name("base32|"))]
    Base32Rfc4648NoPadding,
    #[clap(name("base32:crockford"))]
    Base32Crockford,
    // Base64
    #[clap(name("base64"))]
    Base64Standard,
    #[clap(name("base64|"))]
    Base64StandardNoPadding,
    #[clap(name("base64:url"))]
    Base64UrlSafe,
    #[clap(name("base64:url|"))]
    Base64UrlSafeNoPadding,
    #[clap(name("base64:bcrypt"))]
    Base64Bcrypt,
    #[clap(name("base64:binhex"))]
    Base64Binhex,
    #[clap(name("base64:crypt"))]
    Base64Crypt,
    #[clap(name("base64:imap"))]
    Base64ImapMutf7,
}

impl ToString for Encoding {
    fn to_string(&self) -> String {
        match self {
            Self::Raw => "raw".into(),
            Self::ASCII => "ascii".into(),
            Self::UTF8 => "utf8".into(),
            Self::Hex => "hex".into(),

            Self::Base32Crockford => "base32:crockford".into(),
            Self::Base32Rfc4648 => "base32".into(),
            Self::Base32Rfc4648NoPadding => "base32|".into(),

            Self::Base64Bcrypt => "base64:bcrypt".into(),
            Self::Base64Binhex => "base64:binhex".into(),
            Self::Base64Crypt => "base64:crypt".into(),
            Self::Base64ImapMutf7 => "base64:imap".into(),
            Self::Base64Standard => "base64".into(),
            Self::Base64StandardNoPadding => "base64|".into(),
            Self::Base64UrlSafe => "base64:url".into(),
            Self::Base64UrlSafeNoPadding => "base64:url|".into(),
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

            "base32" => Ok(Self::Base32Rfc4648),
            "base32|" => Ok(Self::Base32Rfc4648NoPadding),
            "base32:crockford" => Ok(Self::Base32Crockford),
            "base32:rfc4648" => Ok(Self::Base32Rfc4648),
            "base32:rfc4648|" => Ok(Self::Base32Rfc4648NoPadding),

            "base64" => Ok(Self::Base64Standard),
            "base64|" => Ok(Self::Base64StandardNoPadding),
            "base64:bcrypt" => Ok(Self::Base64Bcrypt),
            "base64:binhex" => Ok(Self::Base64Binhex),
            "base64:crypt" => Ok(Self::Base64Crypt),
            "base64:imap" => Ok(Self::Base64ImapMutf7),
            "base64:standard" => Ok(Self::Base64Standard),
            "base64:standard|" => Ok(Self::Base64StandardNoPadding),
            "base64:url" => Ok(Self::Base64UrlSafe),
            "base64:url|" => Ok(Self::Base64UrlSafeNoPadding),
            _ => Err(format!("Unknown encoding: {}", s)),
        }
    }
}

#[derive(Debug, Clone, ArgEnum)]
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
    /// The input and output encoding. Encodings that end with `|` do not have padding.
    #[clap(
        arg_enum,
        name = "type",
        number_of_values = 2,
        multiple_occurrences = false
    )]
    pub encoding: Vec<Encoding>,

    /// The file to read from or `-` to read from STDIN
    #[clap(short = 'i', long = "in", default_value = "-")]
    pub input: String,

    /// The encoding of the input (see type arg)
    #[clap(
        arg_enum,
        hide_possible_values(true),
        short = 'I',
        long = "in-type",
        required_unless_present = "type",
        conflicts_with = "type"
    )]
    pub input_type: Option<Encoding>,

    /// The file to write to or `-` to write to STDOUT
    #[clap(short = 'o', long = "out", default_value = "-")]
    pub output: String,

    /// The encoding of the output (see type arg)
    #[clap(
        arg_enum,
        hide_possible_values(true),
        short = 'O',
        long = "out-type",
        required_unless_present = "type",
        conflicts_with = "type"
    )]
    pub output_type: Option<Encoding>,

    /// Controls how data is printed to STDOUT
    #[clap(arg_enum, short = 'F', long = "out-format")]
    pub output_format: Option<OutputFormat>,
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::cli::Args;
    use crate::cli::Encoding;

    #[test]
    fn clap() {
        use clap::CommandFactory;
        Args::command().debug_assert()
    }

    #[test]
    fn serde() {
        let t = |e: Encoding| assert_eq!(e, Encoding::from_str(&e.to_string()).unwrap());

        // Test all long names
        t(Encoding::Raw);
        t(Encoding::ASCII);
        t(Encoding::UTF8);
        t(Encoding::Base32Crockford);
        t(Encoding::Base32Rfc4648);
        t(Encoding::Base32Rfc4648NoPadding);
        t(Encoding::Base64Bcrypt);
        t(Encoding::Base64Binhex);
        t(Encoding::Base64Crypt);
        t(Encoding::Base64ImapMutf7);
        t(Encoding::Base64Standard);
        t(Encoding::Base64StandardNoPadding);
        t(Encoding::Base64UrlSafe);
        t(Encoding::Base64UrlSafeNoPadding);
        t(Encoding::Hex);

        // Test short names
        let t = |e: Encoding, s| assert_eq!(e, Encoding::from_str(s).unwrap());
        t(Encoding::Base32Rfc4648, "base32");
        t(Encoding::Base32Rfc4648NoPadding, "base32|");
        t(Encoding::Base64Standard, "base64");
        t(Encoding::Base64StandardNoPadding, "base64|");
    }
}
