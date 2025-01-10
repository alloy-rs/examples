// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {DynamicExample} from "../src/DynamicExample.sol";
import {Script, console} from "forge-std/Script.sol";

contract CounterScript is Script {

    event log_address(address);

    function setUp() public {} 
    function run() public {
        vm.broadcast();

        DynamicExample s = new DynamicExample();
        emit log_address(address(s));
    }
}
