// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {SP1Verifier} from "./SP1Verifier.sol";

contract MyContract {
    bytes32 public immutable aggregationVkey;
    SP1Verifier public immutable verifier;

    event ProofsVerified(bytes32[] vkeyHashes, bytes[] publicValues);

    constructor(bytes32 _vkeyHash, address _verifier) {
        aggregationVkey = _vkeyHash;
        verifier = SP1Verifier(_verifier);
    }

    function processProofs(bytes memory proof, bytes32[] memory vkeyHashes, bytes[] memory publicValues) public {
        // Write vkeyHashes and publicValues into a single array which we will hash and verify the proof against.
        // The format of the array is ( vkeyHashes.len || vkeyHashes || publicValues[0].len as uint32 || publicValues[0] || ... )
        uint256 totalLength = 4 + vkeyHashes.length * 32 + 4 * publicValues.length;
        for (uint256 i = 0; i < publicValues.length; i++) {
            totalLength += publicValues[i].length;
        }
        bytes memory data = new bytes(totalLength);
        uint256 offset = 0;
        bytes memory vkeyHashesLength = abi.encodePacked(uint32(vkeyHashes.length));
        for (uint256 i = 0; i < 4; i++) {
            data[offset + i] = vkeyHashesLength[i];
        }
        offset += 4;
        for (uint256 i = 0; i < vkeyHashes.length; i++) {
            bytes32 vkeyHash = vkeyHashes[i];
            for (uint256 j = 0; j < 32; j++) {
                data[offset + j] = vkeyHash[j];
            }
            offset += 32;
        }
        for (uint256 i = 0; i < publicValues.length; i++) {
            bytes memory publicValue = publicValues[i];
            uint32 publicValueLength = uint32(publicValue.length);
            bytes memory lengthBytes = abi.encodePacked(publicValueLength);
            for (uint256 j = 0; j < 4; j++) {
                data[offset + j] = lengthBytes[j];
            }
            offset += 4;
            for (uint256 j = 0; j < publicValue.length; j++) {
                data[offset + j] = publicValue[j];
            }
            offset += publicValue.length;
        }

        // Verify the proof against the data
        verifier.verifySP1Proof(aggregationVkey, proof, data);

        // Do something with the vkeyHashes and publicValues
        emit ProofsVerified(vkeyHashes, publicValues);
    }
}
