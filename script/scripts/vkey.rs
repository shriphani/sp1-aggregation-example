use aggregation_lib::words_to_bytes_be;
use sp1_sdk::{mock::MockProver, Prover};

const AGGREGATION_ELF: &[u8] =
    include_bytes!("../../aggregation-program/elf/riscv32im-succinct-zkvm-elf");

/// Generates the vkey digest for the aggregation program.
fn main() {
    sp1_sdk::utils::setup_logger();

    let prover = MockProver::new();
    let (_, vk) = prover.setup(AGGREGATION_ELF);
    let digest = words_to_bytes_be(&vk.hash_u32());
    print!("VKEY_DIGEST={}", hex::encode(digest));
}
