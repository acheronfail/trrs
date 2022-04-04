use anyhow::Result;

use crate::cli::Encoding;

pub fn encode(enc: &Encoding, data: Vec<u8>) -> Result<Vec<u8>> {
    Ok(match enc {
        Encoding::Raw => data,
        Encoding::ASCII => {
            let s = String::from_utf8(data)?;
            if !s.is_ascii() {
                todo!("error here FIXME")
            }

            s.into_bytes()
        }
        Encoding::Hex => hex::encode(&data).into_bytes(),

        Encoding::UTF8 => String::from_utf8(data)?.into_bytes(),
        Encoding::Base32Crockford => {
            base32::encode(base32::Alphabet::Crockford, &data).into_bytes()
        }
        Encoding::Base32Rfc4648 => {
            base32::encode(base32::Alphabet::RFC4648 { padding: true }, &data).into_bytes()
        }
        Encoding::Base32Rfc4648NoPadding => {
            base32::encode(base32::Alphabet::RFC4648 { padding: false }, &data).into_bytes()
        }

        Encoding::Base64Bcrypt => base64::encode_config(&data, base64::BCRYPT).into_bytes(),
        Encoding::Base64Binhex => base64::encode_config(&data, base64::BINHEX).into_bytes(),
        Encoding::Base64Crypt => base64::encode_config(&data, base64::CRYPT).into_bytes(),
        Encoding::Base64ImapMutf7 => base64::encode_config(&data, base64::IMAP_MUTF7).into_bytes(),
        Encoding::Base64Standard => base64::encode_config(&data, base64::STANDARD).into_bytes(),
        Encoding::Base64StandardNoPadding => {
            base64::encode_config(&data, base64::STANDARD_NO_PAD).into_bytes()
        }
        Encoding::Base64UrlSafe => base64::encode_config(&data, base64::URL_SAFE).into_bytes(),
        Encoding::Base64UrlSafeNoPadding => {
            base64::encode_config(&data, base64::URL_SAFE_NO_PAD).into_bytes()
        }
    })
}

#[cfg(test)]
mod test {
    use crate::{cli::Encoding, encode::encode};

    #[test]
    fn it_works() {
        let t = |e: Encoding, inp: &[u8], out: &str| {
            let inp = encode(&e, inp.into()).unwrap();
            let inp = std::str::from_utf8(&inp).unwrap();
            assert_eq!(inp, out);
        };

        let s = "allyourbasearebelongtous".as_bytes();
        t(Encoding::ASCII, s, "allyourbasearebelongtous");
        t(Encoding::UTF8, s, "allyourbasearebelongtous");

        t(
            Encoding::Base32Crockford,
            s,
            "C5P6RYBFENS64RBKCNGQ4SB2CNP6YVK7EHQQAWR",
        );
        t(
            Encoding::Base32Rfc4648,
            s,
            "MFWGY6LPOVZGEYLTMVQXEZLCMVWG63THORXXK4Y=",
        );
        t(
            Encoding::Base32Rfc4648NoPadding,
            s,
            "MFWGY6LPOVZGEYLTMVQXEZLCMVWG63THORXXK4Y",
        );

        {
            // To get padding, the input can't be a multiple of 3
            let s = "allyourbasearebelongtous!".as_bytes();
            t(
                Encoding::Base64Bcrypt,
                s,
                "WUvqcU7zakHfa0TfakTgXUvtZkbyZ1TxGO",
            );
            t(
                Encoding::Base64Binhex,
                s,
                "A9`VG9pdEP*JEe8JEP8KB9`ZDPFcDh8b)3",
            );
            t(
                Encoding::Base64Crypt,
                s,
                "MKlgSKxpQa7VQqJVQaJWNKljPaRoPrJn6E",
            );
            t(
                Encoding::Base64ImapMutf7,
                s,
                "YWxseW91cmJhc2VhcmViZWxvbmd0b3VzIQ",
            );
            t(
                Encoding::Base64Standard,
                s,
                "YWxseW91cmJhc2VhcmViZWxvbmd0b3VzIQ==",
            );
            t(
                Encoding::Base64StandardNoPadding,
                s,
                "YWxseW91cmJhc2VhcmViZWxvbmd0b3VzIQ",
            );
            t(
                Encoding::Base64UrlSafe,
                s,
                "YWxseW91cmJhc2VhcmViZWxvbmd0b3VzIQ==",
            );
            t(
                Encoding::Base64UrlSafeNoPadding,
                s,
                "YWxseW91cmJhc2VhcmViZWxvbmd0b3VzIQ",
            );
        }

        t(
            Encoding::Hex,
            s,
            "616c6c796f75726261736561726562656c6f6e67746f7573",
        );
    }
}
