use crate::core::{RbxReadExt, RbxWriteExt};

#[test]
fn read_interleaved_bytes() {
    #[rustfmt::skip]
    let mut input: &[u8] = &[
        00, 00, 00,
        01, 01, 01,
        02, 02, 02,
        03, 03, 03,
        04, 04, 04,
        05, 05, 05,
        06, 06, 06,
        07, 07, 07,
        08, 08, 08,
        09, 09, 09,
        10, 10, 10,
        11, 11, 11,
        12, 12, 12,
        13, 13, 13,
        14, 14, 14,
        15, 15, 15,
    ];

    let expected = &[
        [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
        [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
        [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
    ];

    let mut result = vec![[0; 16]; expected.len()];
    input.read_interleaved_bytes::<16>(&mut result).unwrap();

    assert_eq!(result, expected)
}

#[test]
fn write_interleaved_bytes() {
    let mut input: &[[u8; 16]] = &[
        [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
        [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
        [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
    ];

    #[rustfmt::skip]
    let expected = &[
            00, 00, 00,
            01, 01, 01,
            02, 02, 02,
            03, 03, 03,
            04, 04, 04,
            05, 05, 05,
            06, 06, 06,
            07, 07, 07,
            08, 08, 08,
            09, 09, 09,
            10, 10, 10,
            11, 11, 11,
            12, 12, 12,
            13, 13, 13,
            14, 14, 14,
            15, 15, 15,
        ];

    let mut result = Vec::new();
    result.write_interleaved_bytes::<16>(&mut input).unwrap();

    assert_eq!(result, expected)
}
