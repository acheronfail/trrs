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
        Encoding::UTF8 => String::from_utf8(data)?.into_bytes(),
        Encoding::Base32 => {
            base32::encode(base32::Alphabet::RFC4648 { padding: true }, &data).into_bytes()
        }
        Encoding::Base64 => base64::encode(&data).into_bytes(),
        Encoding::Hex => hex::encode(&data).into_bytes(),
    })
}

#[cfg(test)]
mod test {
    use crate::{cli::Encoding, encode::encode};

    #[test]
    fn it_works() {
        let s = "allyourbasearebelongtous".as_bytes();
        assert_eq!(
            encode(&Encoding::ASCII, s.into()).unwrap(),
            "allyourbasearebelongtous".as_bytes()
        );
        assert_eq!(
            encode(&Encoding::UTF8, s.into()).unwrap(),
            "allyourbasearebelongtous".as_bytes()
        );
        assert_eq!(
            encode(&Encoding::Base32, s.into()).unwrap(),
            "MFWGY6LPOVZGEYLTMVQXEZLCMVWG63THORXXK4Y=".as_bytes()
        );
        assert_eq!(
            encode(&Encoding::Base64, s.into()).unwrap(),
            "YWxseW91cmJhc2VhcmViZWxvbmd0b3Vz".as_bytes()
        );
        assert_eq!(
            encode(&Encoding::Hex, s.into()).unwrap(),
            "616c6c796f75726261736561726562656c6f6e67746f7573".as_bytes()
        );
    }
}
