use crate::core::{RbxReadExt, RbxWriteInterleaved};

#[rustfmt::skip]
const BYTES_INTERLEAVED: &[u8] = &[
    0, 0, 0,
    1, 1, 1,
    2, 2, 2,
    3, 3, 3,
    4, 4, 4,
    5, 5, 5,
    6, 6, 6,
    7, 7, 7,
    8, 8, 8,
    9, 9, 9,
    10, 10, 10,
    11, 11, 11,
    12, 12, 12,
    13, 13, 13,
    14, 14, 14,
    15, 15, 15,
];

const BYTES_UNINTERLEAVED: [[u8; 16]; 3] = [
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
];

#[test]
fn read_interleaved_bytes() {
    let mut input = BYTES_INTERLEAVED;
    let expected = BYTES_UNINTERLEAVED;

    let mut array_iter = input.read_interleaved_bytes(expected.len()).unwrap();
    let result = core::array::from_fn(|_| array_iter.next().unwrap());

    assert_eq!(result, expected);
    assert!(array_iter.next().is_none());
}

#[test]
fn write_interleaved_bytes() {
    let input = BYTES_UNINTERLEAVED;
    let expected = BYTES_INTERLEAVED;

    let mut result = Vec::new();
    result.write_interleaved_bytes(input).unwrap();

    assert_eq!(result, expected);
}
