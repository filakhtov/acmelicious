static BASE64_ALPHABET: &[u8] =
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_".as_bytes();

pub fn encode(input: &[u8]) -> String {
    let cap = input.len() * 8 / 6 + 1;

    input
        .chunks(3)
        .fold(String::with_capacity(cap), |mut output, quantum| {
            let (remainder, _) = quantum.iter().fold((0, 2), |(remainder, shift), octet| {
                let group = octet >> shift | remainder;
                output.push(BASE64_ALPHABET[group as usize].into());

                let remainder = octet << (8 - shift) >> 2;
                let shift = (shift + 2) % 8;

                (remainder, shift)
            });

            if remainder > 0 {
                output.push(BASE64_ALPHABET[remainder as usize].into());
            }

            output
        })
}

#[cfg(test)]
mod test {
    use super::encode;

    #[test]
    fn test_empty() {
        assert_eq!(encode("".as_bytes()), "");
    }

    #[test]
    fn test_f() {
        assert_eq!(encode("f".as_bytes()), "Zg");
    }

    #[test]
    fn test_fo() {
        assert_eq!(encode("fo".as_bytes()), "Zm8");
    }

    #[test]
    fn test_foo() {
        assert_eq!(encode("foo".as_bytes()), "Zm9v");
    }

    #[test]
    fn test_foob() {
        assert_eq!(encode("foob".as_bytes()), "Zm9vYg");
    }

    #[test]
    fn test_fooba() {
        assert_eq!(encode("fooba".as_bytes()), "Zm9vYmE");
    }

    #[test]
    fn test_foobar() {
        assert_eq!(encode("foobar".as_bytes()), "Zm9vYmFy");
    }
}
