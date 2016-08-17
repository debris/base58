//! Base58-to-text encoding
//! 
//! Based on https://github.com/trezor/trezor-crypto/blob/master/base58.c

const ALPHABET: &'static [u8] = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

/// Errors that can occur when decoding base58 encoded string.
pub enum FromBase58Error {
	/// The input contained a character which is not a part of the base58 format.
	InvalidBase58Characted(char, usize),
	/// The input had invalid length.
	InvalidBase58Length,
}

/// A trait for converting a value to base58 encoded string.
pub trait ToBase58 {
	/// Converts a value of `self` to a base58 value, returning the owned string.
	fn to_base58(&self) -> String;
}

/// A trait for converting base58 encoded values.
pub trait FromBase58 {
	/// Convert a value of `self`, interpreted as base58 encoded data, into an owned vector of bytes, returning a vector.
	fn from_base58(&self) -> Result<Vec<u8>, FromBase58Error>;
}

impl ToBase58 for [u8] {
	fn to_base58(&self) -> String {
		let zcount = self.iter().take_while(|x| **x == 0).count();
		let size = (self.len() - zcount) * 138 / 100 + 1;
		let mut buffer = vec![0u8; size];

		let mut i = zcount;
		let mut high = size - 1;

		while i < self.len() {
			let mut carry = self[i] as u32;
			let mut j = size - 1;

			while j > high || carry != 0 {
				carry += 256 * buffer[j] as u32;
				buffer[j] = (carry % 58) as u8;
				carry /= 58;

				// in original trezor implementation it was underflowing
				if j  > 0 {
					j -= 1;
				}
			}

			i += 1;
			high = j;
		}

		let mut j = buffer.iter().take_while(|x| **x == 0).count();

		let mut result = String::new();
		for _ in 0..zcount {
			result.push('1');
		}

		while j < size {
			result.push(ALPHABET[buffer[j] as usize] as char);
			j += 1;
		}

		result
	}
}

#[cfg(test)]
mod tests {
	use super::ToBase58;

    #[test]
    fn test_to_base58_basic() {
        assert_eq!(b"".to_base58(), "");
        assert_eq!(&[32].to_base58(), "Z");
        assert_eq!(&[45].to_base58(), "n");
        assert_eq!(&[48].to_base58(), "q");
        assert_eq!(&[49].to_base58(), "r");
        assert_eq!(&[57].to_base58(), "z");
        assert_eq!(&[45, 49].to_base58(), "4SU");
        assert_eq!(&[49, 49].to_base58(), "4k8");
        assert_eq!(b"abc".to_base58(), "ZiCa");
        assert_eq!(b"1234598760".to_base58(), "3mJr7AoUXx2Wqd");
        assert_eq!(b"abcdefghijklmnopqrstuvwxyz".to_base58(), "3yxU3u1igY8WkgtjK92fbJQCd4BZiiT1v25f");
    }

    #[test]
    fn test_to_base58_initial_zeros() {
        assert_eq!(b"\0abc".to_base58(), "1ZiCa");
        assert_eq!(b"\0\0abc".to_base58(), "11ZiCa");
        assert_eq!(b"\0\0\0abc".to_base58(), "111ZiCa");
        assert_eq!(b"\0\0\0\0abc".to_base58(), "1111ZiCa");
    }
}
