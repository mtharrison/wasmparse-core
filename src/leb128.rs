use std::io::{Error, ErrorKind, Read};

#[derive(PartialEq)]
enum Sign {
    Signed,
    Unsigned,
}

fn leb128<T: Read + ?Sized>(reader: &mut T, signage: Sign) -> Result<(i64, usize), Error> {
    let n = 32;
    let mut result: i64 = 0;
    let mut shift: usize = 0;
    let mut bytes_read = 0;
    let ceil_bytes = (n as f64 / 7.0).ceil() as usize;

    loop {
        let mut buf = [0];
        reader.read(&mut buf)?;
        let byte = buf[0];
        let low_order_7 = (0b0111_1111 & byte) as i64;
        let hob = (byte >> 7) & 1;

        result |= low_order_7 << shift;
        shift += 7;

        bytes_read += 1;

        if hob == 0 {
            if signage == Sign::Signed {
                let sign_bit_set = (0b0100_0000 & byte) > 0;

                if sign_bit_set {
                    result |= !0 << shift;
                }
            }

            return Ok((result, bytes_read));
        }

        if bytes_read == ceil_bytes {
            return Err(Error::new(
                ErrorKind::Other,
                "No leb128 encoded number found in byte stream",
            ));
        }
    }
}

pub trait ReadLeb128Ext: Read {
    fn leb128_signed(&mut self) -> Result<(i64, usize), Error> {
        leb128(self, Sign::Signed)
    }
    fn leb128_unsigned(&mut self) -> Result<(i64, usize), Error> {
        leb128(self, Sign::Unsigned)
    }
}

impl<R: Read + ?Sized> ReadLeb128Ext for R {}

#[cfg(test)]
mod tests {

    use super::ReadLeb128Ext;
    use std::io::Cursor;

    #[test]
    fn test_leb128_read_unsigned() {
        let bytes = [0xE5, 0x8E, 0x26];
        let mut cursor = Cursor::new(bytes);
        let result = cursor.leb128_unsigned();

        let (number, bytes_read) = result.unwrap();

        assert_eq!(number, 624485);
        assert_eq!(bytes_read, 3);
    }

    #[test]
    fn test_signed_leb128() {
        let bytes = [0x9B, 0xF1, 0x59];
        let mut cursor = Cursor::new(bytes);
        let result = cursor.leb128_signed();

        let (number, bytes_read) = result.unwrap();

        assert_eq!(number, -624485);
        assert_eq!(bytes_read, 3);
    }
}
