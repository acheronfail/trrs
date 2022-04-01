use std::str;

use crate::cli::Encoding;

pub fn decode(enc: &Encoding, data: impl AsRef<[u8]>) -> Vec<u8> {
    let data = data.as_ref();
    match enc {
        Encoding::Raw => data.to_owned(),
        Encoding::ASCII => {
            if data.is_ascii() {
                data.to_owned()
            } else {
                todo!("error here")
            }
        }
        Encoding::UTF8 => data.to_owned(),
        Encoding::Base32 => base32::decode(
            base32::Alphabet::RFC4648 { padding: true },
            str::from_utf8(data).expect("Failed to read base32"),
        )
        .expect("Failed to decode base32"),
        Encoding::Base64 => base64::decode(&data).expect("Failed to decode base64"),
        Encoding::Hex => hex::decode(&data).expect("Failed to decode hex"),
    }
}

#[cfg(test)]
mod test {
    use crate::{cli::Encoding, decode::decode};

    #[test]
    fn it_decodes() {
        let s = "allyourbasearebelongtous".as_bytes();
        assert_eq!(
            decode(&Encoding::ASCII, "allyourbasearebelongtous".as_bytes()),
            s
        );
        assert_eq!(
            decode(&Encoding::UTF8, "allyourbasearebelongtous".as_bytes()),
            s
        );
        assert_eq!(
            decode(
                &Encoding::Base32,
                "MFWGY6LPOVZGEYLTMVQXEZLCMVWG63THORXXK4Y=".as_bytes()
            ),
            s
        );
        assert_eq!(
            decode(
                &Encoding::Base64,
                "YWxseW91cmJhc2VhcmViZWxvbmd0b3Vz".as_bytes()
            ),
            s
        );
        assert_eq!(
            decode(
                &Encoding::Hex,
                "616c6c796f75726261736561726562656c6f6e67746f7573".as_bytes()
            ),
            s
        );
    }
}
