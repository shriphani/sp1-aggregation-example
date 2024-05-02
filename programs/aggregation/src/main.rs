//! This is a test program that takes in sp1_core vkeys and committed values, and then verifies the
//! SP1 proof for each one.

#![no_main]
sp1_zkvm::entrypoint!(main);

use aggregation_lib::{commit_proof_pairs, words_to_bytes_le};
use sha2::Digest;
use sha2::Sha256;
use sp1_zkvm::precompiles::verify::verify_sp1_proof;

pub fn main() {
    let vkeys = sp1_zkvm::io::read::<Vec<[u32; 8]>>();
    let committed_values = sp1_zkvm::io::read::<Vec<Vec<u8>>>();
    assert_eq!(vkeys.len(), committed_values.len());
    committed_values
        .iter()
        .zip(vkeys.iter())
        .enumerate()
        .for_each(|(i, (vals, vkey))| {
            println!("vkeys[{}]: {:?}", i, hex::encode(words_to_bytes_le(vkey)));
            println!("committed_values[{}]: {:?}", i, hex::encode(vals));

            // Get expected pv_digest hash: sha256(input)
            let pv_digest = Sha256::digest(vals);
            verify_sp1_proof(vkey, &pv_digest.into());

            println!("Verified proof for digest: {:?}", hex::encode(pv_digest));
        });

    println!("All {} proofs verified successfully!", vkeys.len());

    // TODO: Do something with vkey / input, ex. build and commit to merkle tree
    // For now we'll just commit to all the (vkey, input) pairs

    let commitment = commit_proof_pairs(&vkeys, &committed_values);
    sp1_zkvm::io::commit_slice(&commitment);
}
