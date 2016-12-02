use super::*;

macro_rules! bin_assert_eq {
    ($x:expr, $y:expr) => {
        let x: u8 = $x.expect("");
        let y: u8 = $y;
        if x != y {
            panic!("assertion failed: `(left == right)` (left: `{:08b}`, right: `{:08b}`)", x, y);
        }
    }
}

#[test]
fn test_first_7() {
    let data = [0b01011011];
    let mut encoder = Encoder::new(&data);
    bin_assert_eq!(encoder.get7(), 0b00101101);
}

#[test]
fn test_second_7() {
    let data = [0b01011011];
    let mut encoder = Encoder::new(&data);
    bin_assert_eq!(encoder.get7(), 0b00101101);
    bin_assert_eq!(encoder.get7(), 0b01000000);
}

#[test]
fn test_middle_7() {
    let data = [0b01011011, 0b10100101];
    let mut encoder = Encoder::new(&data);
    bin_assert_eq!(encoder.get7(), 0b00101101);
    bin_assert_eq!(encoder.get7(), 0b01101001);
}

#[test]
fn test_4_byte_encode() {
    let raw_data = vec![0b10101010, 0b10101010, 0b10101010, 0b10101010];
    let expected =
    vec![HEADER, 0b01010101, 0b00101010, 0b01010101, 0b00101010, 0b01010000];

    let mut encoder = Encoder::new(&raw_data);
    let result = encoder.encode();

    assert_eq!(result, expected);
}
