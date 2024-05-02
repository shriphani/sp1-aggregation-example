//! A simple script to generate and verify the proof of a given program.

use std::str::FromStr;

use aggregation_lib::{commit_proof_pairs, words_to_bytes_be};
use alloy_primitives::U256;
use alloy_sol_macro::sol;
use alloy_sol_types::SolType;
use sp1_sdk::{
    utils::setup_logger, ProverClient, SP1CompressedProof, SP1Groth16Proof, SP1ProvingKey,
    SP1Stdin, SP1VerifyingKey,
};

const AGGREGATION_ELF: &[u8] =
    include_bytes!("../../aggregation-program/elf/riscv32im-succinct-zkvm-elf");
const SIMPLE_ELF: &[u8] = include_bytes!("../../simple-program/elf/riscv32im-succinct-zkvm-elf");

type Uint256_8 = sol! { uint256[8] };

fn run_fibonacci(client: &ProverClient, fib_pk: &SP1ProvingKey, n: u32) -> SP1CompressedProof {
    let mut stdin = SP1Stdin::new();
    stdin.write(&n);
    let mut proof = client
        .prove_compressed(fib_pk, stdin)
        .expect("proving failed");

    let n = proof.public_values.read::<u32>();
    let b = proof.public_values.read::<u128>();
    println!("n: {}", n);
    println!("b: {}", b);

    proof
}

fn aggregate_proofs(
    client: &ProverClient,
    agg_pk: &SP1ProvingKey,
    proofs: Vec<(SP1CompressedProof, SP1VerifyingKey)>,
) -> SP1Groth16Proof {
    let mut stdin = SP1Stdin::new();
    let (vkeys, inputs) = proofs
        .iter()
        .map(|(proof, vkey)| (vkey.hash_u32(), proof.public_values.buffer.data.clone()))
        .unzip();
    stdin.write::<Vec<[u32; 8]>>(&vkeys);
    stdin.write::<Vec<Vec<u8>>>(&inputs);
    for (proof, vkey) in proofs {
        stdin.write_proof(proof.proof, vkey.vk);
    }

    for (i, vkey) in vkeys.iter().enumerate() {
        println!("vkeys[{}]: {:?}", i, hex::encode(words_to_bytes_be(vkey)));
    }

    for (i, input) in inputs.iter().enumerate() {
        println!("inputs[{}]: {:?}", i, hex::encode(input));
    }

    client.prove_groth16(agg_pk, stdin).expect("proving failed")
}

fn main() {
    setup_logger();

    // Setup programs.
    let client = ProverClient::new();
    let (agg_pk, agg_vk) = client.setup(AGGREGATION_ELF);
    let (simple_pk, simple_vk) = client.setup(SIMPLE_ELF);

    let proof_1 =
        tracing::info_span!("fibonacci n=10").in_scope(|| run_fibonacci(&client, &simple_pk, 10));
    let proof_2 =
        tracing::info_span!("fibonacci n=20").in_scope(|| run_fibonacci(&client, &simple_pk, 20));
    let proof_3 =
        tracing::info_span!("fibonacci n=30").in_scope(|| run_fibonacci(&client, &simple_pk, 30));

    let pv_1 = proof_1.public_values.buffer.data.clone();
    let pv_2 = proof_2.public_values.buffer.data.clone();
    let pv_3 = proof_3.public_values.buffer.data.clone();

    let agg_proof = tracing::info_span!("aggregation").in_scope(|| {
        aggregate_proofs(
            &client,
            &agg_pk,
            vec![
                (proof_1, simple_vk.clone()),
                (proof_2, simple_vk.clone()),
                (proof_3, simple_vk.clone()),
            ],
        )
    });

    let vkey_hash = simple_vk.hash_u32();
    let vkeys = vec![vkey_hash, vkey_hash, vkey_hash];
    let pvs = vec![pv_1, pv_2, pv_3];
    let expected_commitment = commit_proof_pairs(&vkeys, &pvs);

    println!("Aggregated proof: {:?}", agg_proof.proof);
    println!(
        "Aggregated commitment: {:?}",
        hex::encode(agg_proof.public_values.buffer.data)
    );
    println!(
        "Expected commitment: {:?}",
        hex::encode(expected_commitment)
    );

    // TODO: this will be moved to sp1_sdk
    let solidity_proof = Uint256_8::abi_encode(&[
        U256::from_str(&agg_proof.proof.a[0]).unwrap(),
        U256::from_str(&agg_proof.proof.a[1]).unwrap(),
        U256::from_str(&agg_proof.proof.b[0][0]).unwrap(),
        U256::from_str(&agg_proof.proof.b[0][1]).unwrap(),
        U256::from_str(&agg_proof.proof.b[1][0]).unwrap(),
        U256::from_str(&agg_proof.proof.b[1][1]).unwrap(),
        U256::from_str(&agg_proof.proof.c[0]).unwrap(),
        U256::from_str(&agg_proof.proof.c[1]).unwrap(),
    ]);
    println!("Solidity proof: {}", hex::encode(solidity_proof));
    println!(
        "Fibonacci vkey: {:?}",
        vkeys
            .iter()
            .map(|vkey| hex::encode(words_to_bytes_be(vkey)))
            .collect::<Vec<_>>()
    );
    println!(
        "Public values: {:?}",
        pvs.iter().map(hex::encode).collect::<Vec<_>>()
    );
    println!(
        "Aggregate vkey: {:?}",
        hex::encode(words_to_bytes_be(&agg_vk.hash_u32()))
    );

    println!("Done!");
}
