# Rust PullServiceClient Readme

The Rust PullServiceClient is designed to interact with a gRPC server for fetching proof data and using that data to
call a smart contract on a blockchain network. This readme provides instructions on how to use the library and customize
certain components for your specific use case.

## Prerequisites

- [Rust](https://www.rust-lang.org/) and [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
  installed on your machine.

# Installation

To use the Rust library for Sui, Aptos and evm follow these steps:

1. Clone the repository or download the library's source code.
2. Navigate to the project directory in your terminal

# Usage

The Rust library for Sui, Aptos and evm provides a complete example that fetches proof data from a gRPC server and then calls a
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
3. Set the chain type evm:
    ```bash
    let chain_type = "evm".to_string();
   ```
4. Set the RPC URL for the desired blockchain network:
    ```bash
    let rpc_url = "<RPC URL>";
   ```

# Customization

Users can customize the smart contract interaction under the call_contract function. Specifically, you can modify the
following components:

1. **Private Key**: Set your private key:
    ```bash
    let secret_key = "<PRIVATE KEY>";
   ```

2. **Contract Address**: Set the address of your smart contract:
    ```bash
    let contract_address = "<CONTRACT ADDRESS>";
   ```

3. **Contract Function Call**: Customize the function call based on your contract methods:
    ```bash
    let call = sc.get_pair_price(Bytes::from(input.proof_bytes), U256::from(0));
   ```

4. **Smart Contract ABI**: Update the path to your smart contract's ABI JSON file and contract name (EVM only)
   in `pull_contract.rs`:
   ```bash
    abigen!(
      MockOracleClient,
      "../../resources/abi.json"
    );
   ```

# Running the Application

Open your terminal and navigate to the project directory.

Run the example using the following command:

**Evm**

```bash
cargo run --example evm_client
```