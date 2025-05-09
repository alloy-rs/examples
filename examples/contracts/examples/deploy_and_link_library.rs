//! Example of deploying a contract that requires [library linking](https://docs.soliditylang.org/en/latest/using-the-compiler.html#library-linking)

use alloy::{
    hex::{FromHex, ToHexExt},
    network::TransactionBuilder,
    primitives::{Address, Bytes, U256},
    providers::ProviderBuilder,
    sol,
};
use eyre::Result;

sol! {
    #[allow(missing_docs)]
    // solc v0.8.26; solc Comparators.sol --optimize --bin
    #[sol(rpc, bytecode = "60808060405234601757609f9081601c823930815050f35b5f80fdfe60808060405260043610156011575f80fd5b5f3560e01c908163118fc88c14604157506321e5749b14602f575f80fd5b60206038366050565b60405191118152f35b602090604b366050565b118152f35b60409060031901126065576004359060243590565b5f80fdfea264697066735822122002fdbd05243d23f18ba1a117f79ab0989e778982b78734134f2dc9c17a49dc7b64736f6c634300081d0033")]
    // credit: <https://github.com/OpenZeppelin/openzeppelin-contracts/blob/fda6b85f2c65d146b86d513a604554d15abd6679/contracts/utils/Comparators.sol>
    library Comparators {
        function lt(uint256 a, uint256 b) internal pure returns (bool) {
            return a < b;
        }

        function gt(uint256 a, uint256 b) internal pure returns (bool) {
            return a > b;
        }
    }

    #[allow(missing_docs)]
    // solc v0.8.26; solc Counter.sol --optimize --bin --libraries Counter.sol:Comparators=0x1234567890123456789012345678901234567890
    // you would get the same result from `forge bind --alloy --libraries Counter.sol:Comparators=0x1234567890123456789012345678901234567890`
    // this `Comparators` address is a placeholder; i.e. you want to deploy it yourself instead of using someone else's.
    #[sol(rpc, bytecode = "60808060405234601557610179908161001a8239f35b5f80fdfe6080806040526004361015610012575f80fd5b5f3560e01c90816380de7e5e146100525750638381f58a14610032575f80fd5b3461004e575f36600319011261004e5760205f54604051908152f35b5f80fd5b3461004e57602036600319011261004e575f54630463f22360e21b8252600482015260043560248201526020816044817312345678901234567890123456789012345678905af4908115610138575f916100d3575b506100ae57005b5f545f1981146100bf576001015f55005b634e487b7160e01b5f52601160045260245ffd5b905060203d602011610131575b601f8101601f1916820167ffffffffffffffff81118382101761011d5760209183916040528101031261004e5751801515810361004e57816100a7565b634e487b7160e01b5f52604160045260245ffd5b503d6100e0565b6040513d5f823e3d90fdfea26469706673582212205e4d98dd89922f8c9b31cbd2509a45e9be6d019d0973432caa5e025347223d7c64736f6c634300081d0033")]
    contract Counter {
        uint256 public number;

        function incrementUntil(uint256 upperBound) public {
            if (Comparators.lt(number, upperBound)) {
                number++;
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    const LIBRARY_PLACEHOLDER_ADDRESS: &str = "1234567890123456789012345678901234567890";

    // Spin up a local Anvil node.
    // Ensure `anvil` is available in $PATH.
    let provider = ProviderBuilder::new().connect_anvil_with_wallet();

    // Deploy the library (instead of using existing ones)
    let lib_addr: Address = Comparators::deploy_builder(&provider).deploy().await?;
    println!("Deployed Comparators library at: {lib_addr}");

    // Link the Counter contract bytecode by replacing the library placeholder
    let counter_linked_bytecode = Bytes::from_hex(
        Counter::BYTECODE.encode_hex().replace(LIBRARY_PLACEHOLDER_ADDRESS, &lib_addr.encode_hex()),
    )?;
    println!("Counter bytecode linked with Comparators library!");

    // Deploy the Counter contract with the linked bytecode
    let counter_addr = Counter::deploy_builder(&provider)
        .map(|req| req.with_deploy_code(counter_linked_bytecode))
        .deploy()
        .await?;
    println!("Deployed Counter contract at: {counter_addr}");

    // Instantiate the deployed Counter contract
    let counter = Counter::new(counter_addr, &provider);

    // Call `incrementUntil(10)` on the contract
    counter.incrementUntil(U256::from(10)).send().await?.watch().await?;
    println!("Counter.incrementUntil(10) invoked!");

    // Assert the counter value is as expected
    let number = counter.number().call().await?;
    assert_eq!(number, U256::from(1));
    println!("Counter.number == 1 verified!");

    Ok(())
}
