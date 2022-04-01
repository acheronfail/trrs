mod cli;
mod decode;
mod encode;

use std::fs::OpenOptions;
use std::io::{self, Read, Write};

use clap::Parser;
use cli::{Args, OutputFormat};

// TODO: nicely formatted error handling for the user with anyhow

fn main() {
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
            stdin
                .lock()
                .read_to_end(&mut stdin_data)
                .expect("Failed to read from STDIN");

            stdin_data
        }
        // Read from file
        _ => {
            let mut data = vec![];
            OpenOptions::new()
                .read(true)
                .open(&args.input)
                .expect(&format!("Failed to open file {}", &args.input))
                .read_to_end(&mut data)
                .expect(&format!("Failed to read file {}", &args.input));
            data
        }
    };

    //
    // Transform
    //

    let data = decode::decode(input_enc, input);
    let output = encode::encode(output_enc, data);

    //
    // Output
    //

    match args.output.as_str() {
        // Print to stdout
        "-" => match args.output_format {
            None | Some(OutputFormat::Raw) => io::stdout()
                .lock()
                .write_all(&output)
                .expect("Failed to write to STDOUT"),
            Some(OutputFormat::Safe) => {
                // TODO: use this instead if https://github.com/sharkdp/bat/pull/2142 is merged
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
                bat.run(input).expect("Failed to print to STDOUT");
            }
        },
        // Write to file
        _ => {
            OpenOptions::new()
                .truncate(true)
                .create(true)
                .write(true)
                .open(&args.output)
                .expect(&format!("Failed to create file {}", &args.output))
                .write_all(&output)
                .expect(&format!("Failed to write file {}", &args.output));
        }
    }
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
                Encoding::Base32,
                base32::encode(base32::Alphabet::RFC4648 { padding: true }, &v.clone())
                    .into_bytes(),
            ),
            (Encoding::Base64, base64::encode(v.clone()).into_bytes()),
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
