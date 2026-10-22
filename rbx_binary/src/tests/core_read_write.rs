use crate::{chunk::ChunkBuilder, core::RbxReadInterleaved, CompressionType};

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

    let result: Vec<_> = input
        .read_interleaved_bytes::<16>(expected.len())
        .unwrap()
        .collect();

    assert_eq!(result, expected)
}

#[test]
fn write_interleaved_bytes() {
    let input: [[u8; 16]; 3] = [
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

    let mut chunk = ChunkBuilder::new(b"ASDF", CompressionType::None);
    chunk.write_interleaved_bytes(input).unwrap();

    let mut dump = Vec::new();
    chunk.dump(&mut dump).unwrap();

    // the first 16 bytes are the chunk header
    let result = &dump[16..];

    assert_eq!(result, expected);
}
