#[macro_use]
extern crate log;

#[cfg(test)]
mod test;

pub const HEADER: u8 = 0b00001111;
const SHORTENED: u8 = 0b01000000;

const DEFAULT_MIME: &'static str = "image/jpg";
const ILLEGALS: [u8; 6] = [0, 10, 13, 34, 38, 92];

pub struct Encoder<'a> {
    data: &'a [u8],
    bit_index: usize,
    byte_index: usize,
}

fn is_illegal(byte: u8) -> Option<u8> {
    for (index, illegal_byte) in ILLEGALS.iter().enumerate() {
        if *illegal_byte == byte {
            return Some(index as u8);
        }
    }

    None
}

fn fmt_buf(data: &[u8]) -> String {
    let mut s = String::with_capacity(data.len() * 8);
    for byte in data.iter() {
        s.push_str(&format!("{:08b}", byte));
    }
    s
}

impl<'a> Encoder<'a> {
    pub fn new(data: &[u8]) -> Encoder {
        Encoder {
            data: data,
            bit_index: 0,
            byte_index: 0,
        }
    }

    fn get7(&mut self) -> Result<u8, ()> {
        if self.byte_index < self.data.len() {
            let first_byte = self.data[self.byte_index];
            let first_part = (((0b11111110 >> self.bit_index) & first_byte) << self.bit_index) >> 1;

            self.bit_index += 7;
            if self.bit_index < 8 {
                Ok(first_part)
            } else {
                self.bit_index -= 8;
                self.byte_index += 1;

                if self.byte_index >= self.data.len() {
                    Ok(first_part)
                } else {
                    let second_byte = self.data[self.byte_index];
                    let second_part = (((0xFF00u16 >> self.bit_index) as u8 & second_byte) &
                                       0xFF) >>
                                      8 - self.bit_index;

                    Ok(first_part | second_part)
                }
            }
        } else {
            Err(())
        }
    }

    pub fn encode(&mut self) -> Vec<u8> {
        let mut buf = Vec::new();
        let mut header = HEADER;
        buf.push(header);

        while let Ok(bits) = self.get7() {
            if let Some(illegal_index) = is_illegal(bits) {
                debug!("Handle illegal sequence: {:08b}", bits);
                let mut b1: u8 = 0b11000010 | (0b111 & illegal_index) << 2;
                let mut b2: u8 = 0b10000000;
                if let Ok(next_bits) = self.get7() {
                    debug!("Additional bits to two - byte character {:08b}", next_bits);

                    let first_bit: u8 = if (next_bits & 0b01000000) > 0 { 1 } else { 0 };
                    b1 |= first_bit;
                    b2 |= next_bits & 0b00111111;
                } else {
                    debug!("Last seven bits are an illegal sequence, and there are no more bits \
                            left");

                    header |= SHORTENED;
                }
                debug!("Adding to buffer: {:08b}{:08b}", b1, b2);
                buf.push(b1);
                buf.push(b2);
            } else {
                debug!("Adding to buffer: {:08b}", bits);
                buf.push(bits);
            }
        }

        debug!("Adding header to buffer: {:08b}", header);
        buf[0] = header;

        debug!("Returning buffer: {}", fmt_buf(&buf));
        buf
    }
}
