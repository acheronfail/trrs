mod cli;
mod decode;
mod encode;

use std::fs::OpenOptions;
use std::io::{self, Read, Write};

use anyhow::{bail, Result};
use clap::Parser;
use cli::{Args, OutputFormat};

fn main() -> Result<()> {
    //
    // Args
    //

    let args = Args::parse();
    let (input_enc, output_enc) = if args.encoding.is_empty() {
        (
            args.input_type.as_ref().unwrap(),
            args.output_type.as_ref().unwrap(),
        )
    } else {
        (&args.encoding[0], &args.encoding[1])
    };

    if args.output_format.is_some() && args.output != "-" {
        eprintln!("Providing --out-format only makes sense when outputting to STDOUT.");
    }

    //
    // Input
    //

    let input = match args.input.as_str() {
        // Read from STDIN
        "-" => {
            let stdin = io::stdin();
            let mut stdin_data = vec![];
            if let Err(e) = stdin.lock().read_to_end(&mut stdin_data) {
                bail!("Failed to read from STDIN: {}", e);
            }

            stdin_data
        }
        // Read from file
        _ => {
            let mut data = vec![];
            match OpenOptions::new().read(true).open(&args.input) {
                Ok(mut file) => {
                    if let Err(e) = file.read_to_end(&mut data) {
                        bail!("Failed to open file: {}", e);
                    }
                }
                Err(e) => bail!("Failed to open file: {}", e),
            }

            data
        }
    };

    //
    // Transform
    //

    let data = decode::decode(input_enc, input)?;
    let output = encode::encode(output_enc, data)?;

    //
    // Output
    //

    match args.output.as_str() {
        // Print to stdout
        "-" => match args.output_format {
            None | Some(OutputFormat::Raw) => {
                if let Err(e) = io::stdout().lock().write_all(&output) {
                    bail!("Failed to write to STDOUT: {}", e)
                }
            }
            Some(OutputFormat::Safe) => {
                // TODO: use this instead when https://github.com/sharkdp/bat/pull/2142 is released
                // bat::PrettyPrinter::new()
                //     .input_from_bytes(&output)
                //     .show_nonprintable(true)
                //     .print()
                //     .expect("Failed to print to STDOUT");

                let mut config = bat::config::Config::default();
                config.show_nonprintable = true;
                let assets = bat::assets::HighlightingAssets::from_binary();
                let bat = bat::controller::Controller::new(&config, &assets);
                let input = vec![bat::input::Input::from(bat::Input::from_bytes(&output))];
                if let Err(e) = bat.run(input) {
                    bail!("Failed to print to STDOUT: {}", e);
                }
            }
        },
        // Write to file
        _ => {
            match OpenOptions::new()
                .truncate(true)
                .create(true)
                .write(true)
                .open(&args.output)
            {
                Ok(mut file) => {
                    if let Err(e) = file.write_all(&output) {
                        bail!("Failed to write file: {}", e)
                    }
                }
                Err(e) => bail!("Failed to create file: {}", e),
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use assert_cmd::Command;

    use crate::cli::Encoding;

    fn cmd() -> Command {
        Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap()
    }

    #[test]
    #[ignore]
    fn error_conditions() {
        todo!()
    }

    #[test]
    #[ignore]
    fn reading_and_writing() {
        todo!()
    }

    #[test]
    fn stdin_and_stdout() {
        let s = {
            let mut s = String::new();
            for i in 0..=0x7f {
                s.push_str(&format!("\\x{:02x}", i));
            }

            s
        };
        let v = s.as_bytes().to_vec();

        let data = vec![
            (Encoding::Raw, v.clone()),
            (Encoding::ASCII, v.clone()),
            (Encoding::UTF8, v.clone()),
            (
                Encoding::Base32Crockford,
                base32::encode(base32::Alphabet::Crockford, &v.clone()).into_bytes(),
            ),
            (
                Encoding::Base32Rfc4648,
                base32::encode(base32::Alphabet::RFC4648 { padding: true }, &v.clone())
                    .into_bytes(),
            ),
            (
                Encoding::Base32Rfc4648NoPadding,
                base32::encode(base32::Alphabet::RFC4648 { padding: false }, &v.clone())
                    .into_bytes(),
            ),
            (
                Encoding::Base64Bcrypt,
                base64::encode_config(v.clone(), base64::BCRYPT).into_bytes(),
            ),
            (
                Encoding::Base64Binhex,
                base64::encode_config(v.clone(), base64::BINHEX).into_bytes(),
            ),
            (
                Encoding::Base64Crypt,
                base64::encode_config(v.clone(), base64::CRYPT).into_bytes(),
            ),
            (
                Encoding::Base64ImapMutf7,
                base64::encode_config(v.clone(), base64::IMAP_MUTF7).into_bytes(),
            ),
            (
                Encoding::Base64Standard,
                base64::encode_config(v.clone(), base64::STANDARD).into_bytes(),
            ),
            (
                Encoding::Base64StandardNoPadding,
                base64::encode_config(v.clone(), base64::STANDARD_NO_PAD).into_bytes(),
            ),
            (
                Encoding::Base64UrlSafe,
                base64::encode_config(v.clone(), base64::URL_SAFE).into_bytes(),
            ),
            (
                Encoding::Base64UrlSafeNoPadding,
                base64::encode_config(v.clone(), base64::URL_SAFE_NO_PAD).into_bytes(),
            ),
            (Encoding::Hex, hex::encode(v.clone()).into_bytes()),
        ];

        for (in_enc, in_data) in &data {
            for (out_enc, out_data) in &data {
                cmd()
                    .args(&["-I", &in_enc.to_string(), "-O", &out_enc.to_string()])
                    .write_stdin(in_data.to_owned())
                    .assert()
                    .success()
                    .stdout(out_data.to_owned());
            }
        }
    }
}
