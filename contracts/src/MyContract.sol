// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {IVerifier} from "./interfaces/IVerifier.sol";

contract MyContract {
    IVerifier public verifier;
    address public verifierKey;

    constructor(address _verifier, address _verifierKey) {
        verifier = IVerifier(_verifier);
        verifierKey = _verifierKey;
    }

    function verifyProof(
        bytes calldata proof,
        uint256[] calldata instances
    ) public view returns (bool) {
        return verifier.verifyProof(verifierKey, proof, instances);
    }
}
