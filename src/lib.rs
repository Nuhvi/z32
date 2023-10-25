//! zero-dependency `no-std` compatible MIT licensed encoding and decoding of z-bas32.
//!
//! This is an implementation of the human-oriented base-32 encoding called
//! [z-base32](https://philzimmermann.com/docs/human-oriented-base-32-encoding.txt).

/// Alphabet used by zbase32
pub const ALPHABET: &[u8; 32] = b"ybndrfg8ejkmcpqxot1uwisza345h769";

/// Encode first N `bits` with zbase32.
///
/// # Panics
///
/// Panics if `data` is shorter than N `bits`.
///
/// # Examples
///
/// ```
/// use z32;
///
/// let data = "The quick brown fox jumps over the lazy dog. 👀";
/// assert_eq!(z32::encode(data.as_bytes(), 64), "ktwgkedtqiwsg");
/// ```
///
pub fn encode(buf: &[u8], bits: usize) -> String {
    let capacity = if bits % 5 == 0 {
        bits / 5
    } else {
        bits / 5 + 1
    } as usize;

    let mut s = Vec::with_capacity(capacity);

    for p in (0..bits).step_by(5) {
        let i = p >> 3;
        let j = p & 7;
        if j <= 3 {
            s.push(ALPHABET[((buf[i] >> (3 - j)) & 0b11111) as usize]);
        } else {
            let of = j - 3;
            let h = (buf[i] << of) & 0b11111;
            let l = if i >= buf.len() - 1 {
                0
            } else {
                buf[i + 1] >> (8 - of)
            };
            s.push(ALPHABET[(h | l) as usize]);
        }
    }

    unsafe { String::from_utf8_unchecked(s) }
}

/// Encode full bytes using zbase32.
///
/// Just like `encode` but doesn't allow encoding with bit precision.
///
/// # Examples
///
/// ```
/// use z32;
///
/// let data = "The quick brown fox jumps over the lazy dog. 👀";
/// assert_eq!(z32::encode_full_bytes(data.as_bytes()),
///            "ktwgkedtqiwsg43ycj3g675qrbug66bypj4s4hdurbzzc3m1rb4go3jyptozw6jyctzsqmty6nx3dyy");
/// ```
pub fn encode_full_bytes(buf: &[u8]) -> String {
    encode(buf, buf.len() * 8)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic() {
        let input = "The quick brown fox jumps over the lazy dog. 👀";

        {
            let encoded = encode_full_bytes(input.as_bytes());
            assert_eq!(
                encoded,
                "ktwgkedtqiwsg43ycj3g675qrbug66bypj4s4hdurbzzc3m1rb4go3jyptozw6jyctzsqmty6nx3dyy"
            );
        }

        // let decoded = decode_full_bytes_str(&encoded);
        // println!("Decoded: {:?}", decoded);
        //

        {
            let encoded = encode(input.as_bytes(), 64);
            assert_eq!(encoded, "ktwgkedtqiwsg");
        }
    }

    // #[test]
    // fn random() {
    //     let mut rng = rand::thread_rng();
    //     let random_bytes: [u8; 20] = rng.gen();
    // }
}
