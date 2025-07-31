use std::{fmt::Debug, fmt::Display, u8};

/// Data structure for storing vector of bits
///

#[derive(Debug, PartialEq)]
pub enum Bit {
    Zero,
    One,
}

impl TryFrom<u8> for Bit {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Bit::Zero),
            1 => Ok(Bit::One),
            _ => Err("Bit can only accept 0 or 1!"),
        }
    }
}

impl Display for Bit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = match self {
            Bit::Zero => "0",
            Bit::One => "1",
        };
        write!(f, "{}", v)
    }
}

pub fn to_bytes<'a, I>(value: I) -> Vec<u8>
where
    I: Iterator<Item = Bit> + Debug,
{
    let mut bytes: Vec<u8> = vec![];

    for (k, bit) in value.enumerate() {
        let byte_pos: usize = k % 8;
        if byte_pos == 0 {
            bytes.push(0);
        }

        let byte_index = k / 8;

        match bit {
            Bit::One => bytes[byte_index] |= 0b1000_0000 >> byte_pos,
            _ => (),
        }
    }

    bytes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bit_create() {
        assert_eq!(Bit::try_from(0), Ok(Bit::Zero));
        assert_eq!(Bit::try_from(1), Ok(Bit::One));
        assert_eq!(Bit::try_from(3), Err("Bit can only accept 0 or 1!"));
    }

    #[test]
    fn test_to_bytes() {
        assert_eq!(to_bytes(vec![Bit::One].into_iter()), vec![128]);
        assert_eq!(to_bytes(vec![Bit::One, Bit::One].into_iter()), vec![192]);
        assert_eq!(
            to_bytes(
                vec![
                    Bit::One,
                    Bit::One,
                    Bit::Zero,
                    Bit::Zero,
                    Bit::Zero,
                    Bit::Zero,
                    Bit::Zero,
                    Bit::Zero,
                    Bit::One,
                    Bit::Zero,
                    Bit::Zero,
                    Bit::Zero,
                    Bit::Zero,
                    Bit::Zero,
                    Bit::Zero,
                    Bit::Zero,
                    Bit::Zero,
                    Bit::Zero,
                    Bit::Zero,
                    Bit::Zero,
                    Bit::Zero,
                    Bit::Zero,
                    Bit::Zero,
                    Bit::One
                ]
                .into_iter()
            ),
            vec![192, 128, 1]
        );
    }
}
