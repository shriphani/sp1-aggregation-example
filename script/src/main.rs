//! A simple script to generate and verify the proof of a given program.

use sp1_sdk::{
    utils::setup_logger, ProverClient, SP1CompressedProof, SP1Groth16Proof, SP1ProvingKey,
    SP1Stdin, SP1VerifyingKey,
};

const AGGREGATION_ELF: &[u8] =
    include_bytes!("../../aggregation-program/elf/riscv32im-succinct-zkvm-elf");
const SIMPLE_ELF: &[u8] = include_bytes!("../../simple-program/elf/riscv32im-succinct-zkvm-elf");

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

    client.prove_groth16(agg_pk, stdin).expect("proving failed")
}

fn main() {
    setup_logger();

    // Setup programs.
    let client = ProverClient::new();
    let (agg_pk, _agg_vk) = client.setup(AGGREGATION_ELF);
    let (simple_pk, simple_vk) = client.setup(SIMPLE_ELF);

    let proof_1 =
        tracing::info_span!("fibonacci n=10").in_scope(|| run_fibonacci(&client, &simple_pk, 10));
    let proof_2 =
        tracing::info_span!("fibonacci n=20").in_scope(|| run_fibonacci(&client, &simple_pk, 20));
    let proof_3 =
        tracing::info_span!("fibonacci n=30").in_scope(|| run_fibonacci(&client, &simple_pk, 30));

    let agg_proof = tracing::info_span!("aggregation").in_scope(|| {
        aggregate_proofs(
            &client,
            &agg_pk,
            vec![
                (proof_1, simple_vk.clone()),
                (proof_2, simple_vk.clone()),
                (proof_3, simple_vk),
            ],
        )
    });

    println!("Aggregated proof: {:?}", agg_proof.proof);

    let a = agg_proof.proof.a;
    println!("Done!");
}
