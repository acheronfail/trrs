use std::str;

use anyhow::{bail, Result};

use crate::cli::Encoding;

fn base32(alphabet: base32::Alphabet, data: &[u8]) -> Result<Vec<u8>> {
    match base32::decode(alphabet, str::from_utf8(data)?) {
        Some(r) => Ok(r),
        None => bail!("Failed to decode base32"),
    }
}

pub fn decode(enc: &Encoding, data: impl AsRef<[u8]>) -> Result<Vec<u8>> {
    let data = data.as_ref();
    Ok(match enc {
        Encoding::Raw => data.to_owned(),
        Encoding::ASCII => {
            if data.is_ascii() {
                data.to_owned()
            } else {
                todo!("error here FIXME")
            }
        }
        Encoding::UTF8 => data.to_owned(),
        Encoding::Hex => hex::decode(&data)?,

        Encoding::Base32Crockford => base32(base32::Alphabet::Crockford, &data)?,
        Encoding::Base32Rfc4648 => base32(base32::Alphabet::RFC4648 { padding: true }, &data)?,
        Encoding::Base32Rfc4648NoPadding => {
            base32(base32::Alphabet::RFC4648 { padding: false }, &data)?
        }

        Encoding::Base64Bcrypt => base64::decode_config(&data, base64::BCRYPT)?,
        Encoding::Base64Binhex => base64::decode_config(&data, base64::BINHEX)?,
        Encoding::Base64Crypt => base64::decode_config(&data, base64::CRYPT)?,
        Encoding::Base64ImapMutf7 => base64::decode_config(&data, base64::IMAP_MUTF7)?,
        Encoding::Base64Standard => base64::decode_config(&data, base64::STANDARD)?,
        Encoding::Base64StandardNoPadding => base64::decode_config(&data, base64::STANDARD_NO_PAD)?,
        Encoding::Base64UrlSafe => base64::decode_config(&data, base64::URL_SAFE)?,
        Encoding::Base64UrlSafeNoPadding => base64::decode_config(&data, base64::URL_SAFE_NO_PAD)?,
    })
}

#[cfg(test)]
mod test {
    use crate::{cli::Encoding, decode::decode};

    #[test]
    fn it_decodes() {
        let t = |e: Encoding, inp: &[u8], out: &str| {
            let inp = decode(&e, inp).unwrap();
            let inp = std::str::from_utf8(&inp).unwrap();
            assert_eq!(inp, out);
        };

        let s = "allyourbasearebelongtous";
        t(Encoding::ASCII, b"allyourbasearebelongtous", s);
        t(Encoding::UTF8, b"allyourbasearebelongtous", s);

        t(
            Encoding::Base32Crockford,
            b"C5P6RYBFENS64RBKCNGQ4SB2CNP6YVK7EHQQAWR",
            s,
        );
        t(
            Encoding::Base32Rfc4648,
            b"MFWGY6LPOVZGEYLTMVQXEZLCMVWG63THORXXK4Y=",
            s,
        );
        t(
            Encoding::Base32Rfc4648NoPadding,
            b"MFWGY6LPOVZGEYLTMVQXEZLCMVWG63THORXXK4Y",
            s,
        );
        {
            let s = "allyourbasearebelongtous!";
            t(
                Encoding::Base64Bcrypt,
                b"WUvqcU7zakHfa0TfakTgXUvtZkbyZ1TxGO",
                s,
            );
            t(
                Encoding::Base64Binhex,
                b"A9`VG9pdEP*JEe8JEP8KB9`ZDPFcDh8b)3",
                s,
            );
            t(
                Encoding::Base64Crypt,
                b"MKlgSKxpQa7VQqJVQaJWNKljPaRoPrJn6E",
                s,
            );
            t(
                Encoding::Base64ImapMutf7,
                b"YWxseW91cmJhc2VhcmViZWxvbmd0b3VzIQ",
                s,
            );
            t(
                Encoding::Base64Standard,
                b"YWxseW91cmJhc2VhcmViZWxvbmd0b3VzIQ==",
                s,
            );
            t(
                Encoding::Base64StandardNoPadding,
                b"YWxseW91cmJhc2VhcmViZWxvbmd0b3VzIQ",
                s,
            );
            t(
                Encoding::Base64UrlSafe,
                b"YWxseW91cmJhc2VhcmViZWxvbmd0b3VzIQ==",
                s,
            );
            t(
                Encoding::Base64UrlSafeNoPadding,
                b"YWxseW91cmJhc2VhcmViZWxvbmd0b3VzIQ",
                s,
            );
        }

        t(
            Encoding::Hex,
            b"616c6c796f75726261736561726562656c6f6e67746f7573",
            s,
        );
    }
}
