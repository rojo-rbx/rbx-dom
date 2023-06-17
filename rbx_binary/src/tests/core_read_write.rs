use crate::core::{RbxReadExt, RbxWriteExt};

#[test]
fn read_interleaved_bytes() {
    #[rustfmt::skip]
    let mut input: &[u8] = &[
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
    let input: &[[u8; 16]] = &[
        [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
        [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
        [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
    ];

    #[rustfmt::skip]
    let expected = &[
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

    let mut result = Vec::new();
    result.write_interleaved_bytes::<16>(input).unwrap();

    assert_eq!(result, expected)
}
