// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script, console} from "forge-std/Script.sol";
import {SP1Verifier} from "../src/SP1Verifier.sol";
import {MyContract} from "../src/MyContract.sol";

contract Deploy is Script {
    function setUp() public {}

    function run() public {
        // Call `cargo run --release --bin vkey` in script to get the vkey hash

        bytes32 vkeyHash = vm.envBytes32("VKEY_DIGEST");
        address verifierAddress = vm.envOr("SP1_VERIFIER_ADDRESS", address(0));

        console.log("VKEY_DIGEST:");
        console.logBytes32(vkeyHash);
        console.log("SP1_VERIFIER_ADDRESS:");
        console.log(verifierAddress);
        console.log("");

        vm.startBroadcast();
        if (verifierAddress == address(0)) {
            console.log("Deploying SP1Verifier");
            verifierAddress = address(new SP1Verifier());
            console.log("SP1Verifier:", verifierAddress);
        }

        console.log("Deploying MyContract");
        address myContract = address(new MyContract(vkeyHash, verifierAddress));
        console.log("MyContract:", myContract);
    }
}
