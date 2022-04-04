use std::str;

use anyhow::{bail, Result};

use crate::cli::Encoding;

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
        Encoding::Base32 => match base32::decode(
            base32::Alphabet::RFC4648 { padding: true },
            str::from_utf8(data)?,
        ) {
            Some(r) => r,
            None => bail!("Failed to decode base32"),
        },
        Encoding::Base64 => base64::decode(&data)?,
        Encoding::Hex => hex::decode(&data)?,
    })
}

#[cfg(test)]
mod test {
    use crate::{cli::Encoding, decode::decode};

    #[test]
    fn it_decodes() {
        let s = "allyourbasearebelongtous".as_bytes();
        assert_eq!(
            decode(&Encoding::ASCII, "allyourbasearebelongtous".as_bytes()).unwrap(),
            s
        );
        assert_eq!(
            decode(&Encoding::UTF8, "allyourbasearebelongtous".as_bytes()).unwrap(),
            s
        );
        assert_eq!(
            decode(
                &Encoding::Base32,
                "MFWGY6LPOVZGEYLTMVQXEZLCMVWG63THORXXK4Y=".as_bytes()
            )
            .unwrap(),
            s
        );
        assert_eq!(
            decode(
                &Encoding::Base64,
                "YWxseW91cmJhc2VhcmViZWxvbmd0b3Vz".as_bytes()
            )
            .unwrap(),
            s
        );
        assert_eq!(
            decode(
                &Encoding::Hex,
                "616c6c796f75726261736561726562656c6f6e67746f7573".as_bytes()
            )
            .unwrap(),
            s
        );
    }
}
