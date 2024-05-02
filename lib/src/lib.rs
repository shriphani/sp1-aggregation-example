pub fn words_to_bytes_le(words: &[u32; 8]) -> [u8; 32] {
    let mut bytes = [0u8; 32];
    for i in 0..8 {
        let word_bytes = words[i].to_le_bytes();
        bytes[i * 4..(i + 1) * 4].copy_from_slice(&word_bytes);
    }
    bytes
}

pub fn words_to_bytes_be(words: &[u32; 8]) -> [u8; 32] {
    let mut bytes = [0u8; 32];
    for i in 0..8 {
        let word_bytes = words[i].to_be_bytes();
        bytes[i * 4..(i + 1) * 4].copy_from_slice(&word_bytes);
    }
    bytes
}

/// Encode a list of vkeys and committed values into a single byte array. In the future this could
/// be a merkle tree or some other commitment scheme.
///
/// ( vkeys.len() || vkeys || committed_values[0].len as u32 || committed_values[0] || ... )
pub fn commit_proof_pairs(vkeys: &[[u32; 8]], committed_values: &[Vec<u8>]) -> Vec<u8> {
    assert_eq!(vkeys.len(), committed_values.len());
    let mut res = Vec::with_capacity(
        4 + vkeys.len() * 32
            + committed_values.len() * 4
            + committed_values
                .iter()
                .map(|vals| vals.len())
                .sum::<usize>(),
    );

    // Note we use big endian because abi.encodePacked in solidity does also
    res.extend_from_slice(&(vkeys.len() as u32).to_be_bytes());
    for vkey in vkeys.iter() {
        res.extend_from_slice(&words_to_bytes_le(vkey));
    }
    for vals in committed_values.iter() {
        res.extend_from_slice(&(vals.len() as u32).to_be_bytes());
        res.extend_from_slice(vals);
    }

    res
}
