// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

contract DynamicExample {
    string name;
    event Named(string s);

    function setName(string calldata s) public {
        name = s;
        emit Named(s);
    }
}
