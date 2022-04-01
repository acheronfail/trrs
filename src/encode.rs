use crate::cli::Encoding;

pub fn encode(enc: &Encoding, data: Vec<u8>) -> Vec<u8> {
    match enc {
        Encoding::Raw => data,
        Encoding::ASCII => String::from_utf8(data)
            .expect("Failed to encode ASCII")
            .into_bytes(),
        Encoding::UTF8 => String::from_utf8(data)
            .expect("Failed to encode UTF-8")
            .into_bytes(),
        Encoding::Base32 => {
            base32::encode(base32::Alphabet::RFC4648 { padding: true }, &data).into_bytes()
        }
        Encoding::Base64 => base64::encode(&data).into_bytes(),
        Encoding::Hex => hex::encode(&data).into_bytes(),
    }
}

#[cfg(test)]
mod test {
    use crate::{cli::Encoding, encode::encode};

    #[test]
    fn it_works() {
        let s = "allyourbasearebelongtous".as_bytes();
        assert_eq!(
            encode(&Encoding::ASCII, s.into()),
            "allyourbasearebelongtous".as_bytes()
        );
        assert_eq!(
            encode(&Encoding::UTF8, s.into()),
            "allyourbasearebelongtous".as_bytes()
        );
        assert_eq!(
            encode(&Encoding::Base32, s.into()),
            "MFWGY6LPOVZGEYLTMVQXEZLCMVWG63THORXXK4Y=".as_bytes()
        );
        assert_eq!(
            encode(&Encoding::Base64, s.into()),
            "YWxseW91cmJhc2VhcmViZWxvbmd0b3Vz".as_bytes()
        );
        assert_eq!(
            encode(&Encoding::Hex, s.into()),
            "616c6c796f75726261736561726562656c6f6e67746f7573".as_bytes()
        );
    }
}
