//! Example for batching multiple UniswapV2Factory reads using the Multicall3 contract.
//!
//!
use alloy::primitives::{address, Address, FixedBytes, U256};
use alloy::providers::ProviderBuilder;
use alloy::sol;
use alloy::transports::http::reqwest::Url;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    Multicall3,
    "examples/abi/Multicall3.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    UniswapV2Factory,
    "examples/abi/UniswapV2Factory.json"
);

const RPC_URL: &str = "https://eth.merkle.io";
const UNIV2_FACTORY: Address = address!("5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f");
const MULTICALL_ADDR: Address = address!("cA11bde05977b3631167028862bE2a173976CA11");

const BATCH_SIZE: usize = 10;

#[tokio::main]
async fn main() {
    let rpc_url = RPC_URL.parse::<Url>().unwrap();
    let provider = ProviderBuilder::new().on_http(rpc_url);

    let multicall = Multicall3::new(MULTICALL_ADDR, provider.clone());
    let factory = UniswapV2Factory::new(UNIV2_FACTORY, provider);

    let calls = (0..BATCH_SIZE)
        .map(|index| Multicall3::Call3 {
            target: UNIV2_FACTORY,
            allowFailure: false,
            callData: factory.allPairs(U256::from(index)).calldata().to_owned(),
        })
        .collect();

    let pools: Vec<Address> = multicall
        .aggregate3(calls)
        .call()
        .await
        .unwrap()
        .returnData
        .iter()
        .map(to_address)
        .collect();

    println!("{pools:?}");
}

fn to_address(res: &Multicall3::Result) -> Address {
    let bytes = FixedBytes::<32>::from_slice(&res.returnData);
    Address::from_word(bytes)
}
