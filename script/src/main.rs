//! A simple example showing how to aggregate proofs of multiple programs with SP1.

use std::str::FromStr;

use alloy_primitives::U256;
use alloy_sol_macro::sol;
use alloy_sol_types::SolType;
use sp1_sdk::{ProverClient, SP1CompressedProof, SP1Stdin, SP1VerifyingKey};

/// A program that aggregates the proofs of the simple program.
const AGGREGATION_ELF: &[u8] =
    include_bytes!("../../programs/aggregation/elf/riscv32im-succinct-zkvm-elf");

/// A program that just runs a simple computation.
const FIBONACCI_ELF: &[u8] =
    include_bytes!("../../programs/fibonacci/elf/riscv32im-succinct-zkvm-elf");

/// An input to the aggregation program.
/// 
/// Consists of a proof and a verification key.
struct AggregationInput {
    pub proof: SP1CompressedProof,
    pub vk: SP1VerifyingKey,
}

fn main() {
    // Setup the logger.
    sp1_sdk::utils::setup_logger();

    // Initialize the proving client.
    let client = ProverClient::new();

    // Setup the proving and verifying keys.
    let (aggregation_pk, _) = client.setup(AGGREGATION_ELF);
    let (fibonacci_pk, fibonacci_vk) = client.setup(FIBONACCI_ELF);

    // Generate the fibonacci proofs.
    let proof_1 = tracing::info_span!("generate fibonacci proof n=10").in_scope(|| {
        let mut stdin = SP1Stdin::new();
        stdin.write(&10);
        client
            .prove_compressed(&fibonacci_pk, stdin)
            .expect("proving failed")
    });
    let proof_2 = tracing::info_span!("generate fibonacci proof n=20").in_scope(|| {
        let mut stdin = SP1Stdin::new();
        stdin.write(&20);
        client
            .prove_compressed(&fibonacci_pk, stdin)
            .expect("proving failed")
    });
    let proof_3 = tracing::info_span!("generate fibonacci proof n=30").in_scope(|| {
        let mut stdin = SP1Stdin::new();
        stdin.write(&30);
        client
            .prove_compressed(&fibonacci_pk, stdin)
            .expect("proving failed")
    });

    // Setup the inputs to the aggregation program.
    let input_1 = AggregationInput {
        proof: proof_1,
        vk: fibonacci_vk.clone(),
    };
    let input_2 = AggregationInput {
        proof: proof_2,
        vk: fibonacci_vk.clone(),
    };
    let input_3 = AggregationInput {
        proof: proof_3,
        vk: fibonacci_vk.clone(),
    };
    let inputs = vec![input_1, input_2, input_3];

    // Aggregate the proofs.
    let aggregated_proof = tracing::info_span!("aggregate the proofs").in_scope(|| {
        let mut stdin = SP1Stdin::new();

        // Write the verification keys.
        let vkeys = inputs
            .iter()
            .map(|input| input.vk.hash_u32())
            .collect::<Vec<_>>();
        stdin.write::<Vec<[u32; 8]>>(&vkeys);

        // Write the public values.
        let public_values = inputs
            .iter()
            .map(|input| input.proof.public_values.buffer.data.clone())
            .collect::<Vec<_>>();
        stdin.write::<Vec<Vec<u8>>>(&public_values);

        // Write the proofs.
        //
        // Note: this data will not actually be read by the aggregation program, instead it will be
        // witnessed by the prover during the recursive aggregation process inside SP1 itself.
        for input in inputs {
            stdin.write_proof(input.proof.proof, input.vk.vk);
        }

        // Generate the groth16 proof.
        client
            .prove_groth16(&aggregation_pk, stdin)
            .expect("proving failed")
    });

    // Print the results.
    type Uint256_8 = sol! { uint256[8] };
    let solidity_groth16_proof = Uint256_8::abi_encode(&[
        U256::from_str(&aggregated_proof.proof.a[0]).unwrap(),
        U256::from_str(&aggregated_proof.proof.a[1]).unwrap(),
        U256::from_str(&aggregated_proof.proof.b[0][0]).unwrap(),
        U256::from_str(&aggregated_proof.proof.b[0][1]).unwrap(),
        U256::from_str(&aggregated_proof.proof.b[1][0]).unwrap(),
        U256::from_str(&aggregated_proof.proof.b[1][1]).unwrap(),
        U256::from_str(&aggregated_proof.proof.c[0]).unwrap(),
        U256::from_str(&aggregated_proof.proof.c[1]).unwrap(),
    ]);
    println!(
        "solidity groth16 proof: {}",
        hex::encode(solidity_groth16_proof)
    );
}
