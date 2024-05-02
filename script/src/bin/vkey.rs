//! Exports the vkey digest for the current wrap circuit.

use aggregation_lib::words_to_bytes_be;
use sp1_sdk::{mock::MockProver, Prover};

const AGGREGATION_ELF: &[u8] =
    include_bytes!("../../../programs/aggregation/elf/riscv32im-succinct-zkvm-elf");

fn main() {
    sp1_sdk::utils::setup_logger();
    let prover = MockProver::new();
    let (_, vk) = prover.setup(AGGREGATION_ELF);
    let digest = words_to_bytes_be(&vk.hash_u32());
    print!("vkey: {}", hex::encode(digest));
}
