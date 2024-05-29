// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

interface IVerifier {
    function verifyProof(
        address vk,
        bytes calldata proof,
        uint256[] calldata instances
    ) external view returns (bool);
}
