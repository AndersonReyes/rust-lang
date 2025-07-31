use std::{fs, io::Write};

use rath::{bit_vector::to_bytes, huffman_codes};

#[test]
fn alice_in_wonderland_test() {
    let data = fs::read_to_string("assets/alice_in_wonderland.txt").unwrap();

    let tree = huffman_codes::build(&data);

    let encoded = huffman_codes::encode_data2(&data, &tree);

    let mut file = fs::File::create("assets/alice_in_wonderland_encoded.txt").unwrap();

    assert_eq!(huffman_codes::decode_bits(encoded.iter(), &tree), data);

    file.write_all(&to_bytes(encoded.into_iter())).unwrap();
}
