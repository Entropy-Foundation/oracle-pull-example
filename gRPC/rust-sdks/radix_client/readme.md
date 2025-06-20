# Rust PullServiceClient Readme

The Rust PullServiceClient is designed to interact with a gRPC server for fetching proof data and using that data to
call a smart contract on a blockchain network. This readme provides instructions on how to use the library and customize
certain components for your specific use case.

## Prerequisites

- [Rust](https://www.rust-lang.org/) and [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) installed on your machine.

# Installation

To use the Rust library for Radix, Sui, Aptos or EVM follow these steps:

1. Clone the repository or download the library's source code.
2. Navigate to the project directory in your terminal

# Usage

The Rust library for Radix, Sui, Aptos or EVM provides a complete example that fetches proof data from a gRPC server and then calls a
contract function on a blockchain network.

# Configuration

Before using the library, configure the file in example folder:

1. Set the gRPC server address:

   **Testnets**
    ```bash
    let address = "https://testnet-dora-2.supra.com:443".to_string();
   ```

2. Set the pair indexes as an array:
    ```bash
    let pair_indexes = vec![0, 21, 61, 49];
    ```
3. Set the chain type radix:
    ```bash
    let chain_type = "radix".to_string();
   ```
4. Set the NetworkConfig for the desired radix network in `radix_connector.rs` eg. Stokenet:
   ```bash
   const GATEWAY_URL = "https://stokenet.radixdlt.com";
   const NETWORK_ID : u8 = 2;
   const LOGICAL_NAME: &str = "stokenet";
   const HRP_SUFFIX: &str = "tdx_2_";
   ```

# Customization

Users can customize the smart contract interaction under the `invoke_radix_chain` function. Specifically, you can modify the
following components:

1. **Private Key**: Set your private key:
    ```bash
    let private_key = Ed25519PrivateKey::from_bytes(&hex::decode("<PRIVATE_KEY>").unwrap()).unwrap();;
   ```

2. **Component Address**: Set the address of your component smart contract:
    ```bash
    let component_address =
        ComponentAddress::try_from_bech32(&address_decoder, "<COMPONENT_ADDRESS>")
            .expect("Invalid component address");
   ```

3. **Component Function Call**: Customize the function call based on your contract methods:
    ```bash
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .call_method(
            DynamicGlobalAddress::from(component_address),
            "<COMPONENT METHOD>",
            manifest_args!(oracle_proof_bytes),
        )
        .build();
   ```

# Running the Application

Open your terminal and navigate to the project directory.

Run the example using the following command:

**Radix**

```bash
cargo run --example radix_client
```