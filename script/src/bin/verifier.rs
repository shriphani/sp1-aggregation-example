//! Exports the solidity verifier to the contracts folder.

use std::path::PathBuf;

use sp1_sdk::artifacts::{export_solidity_verifier, WrapCircuitType};

fn main() {
    sp1_sdk::utils::setup_logger();

    export_solidity_verifier(
        WrapCircuitType::Groth16,
        PathBuf::from("../contracts/src"),
        None,
    )
    .expect("failed to export verifier");
}
