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
    ```bash
    let address = "grpcs:://testnet-dora.supraoracles.com".to_string();
   ```
2. Set the pair indexes as an array:
    ```bash
    let pair_indexes = vec![0, 21, 61, 49];
    ```
3. Set the chain type sui:
    ```bash
    let chain_type = "sui".to_string();
   ```
4. Set the RPC URL for the desired blockchain network:
    ```bash
    “<--rpc-url-->”;
   ```

# Customization

Users can customize the smart contract interaction under the call_contract function. Specifically, you can modify the
following components:

1. **Private Key**: Set your private key:
    ```bash
    "<--secret-key-->";
   ```

2. **Contract Address**: Set the address of your smart contract:
    ```bash
    "<-contract-address-->";
   ```

3. **Contract Function Call**: Customize the function call based on your contract methods:
    ```bash
    const MODULE: &str = "<CONTRACT MODULE>";
    const ENTRY: &str = "<CONTRACT FUNCTION>";
   ```

5. **Transaction Object**: Customize the transaction object as needed:
    ```bash
    txb.moveCall({
      target: `${contractAddress}::${moduleName}::${functionName}`,
      arguments: [
        txb.pure(response.dkg_object),
        txb.pure(response.oracle_holder_object),
        txb.pure(response.vote_smr_block_round, "vector<vector<u8>>"),
        txb.pure(response.vote_smr_block_timestamp, "vector<vector<u8>>"),
        txb.pure(response.vote_smr_block_author, "vector<vector<u8>>"),
        txb.pure(response.vote_smr_block_qc_hash, "vector<vector<u8>>"),
        txb.pure(response.vote_smr_block_batch_hashes, "vector<vector<u8>>"),
        txb.pure(response.vote_round, "vector<u64>"),
        txb.pure(response.min_batch_protocol, "vector<vector<u8>>"),
        txb.pure(response.min_batch_txn_hashes, "vector<vector<vector<u8>>>"),
        txb.pure(response.min_txn_cluster_hashes, "vector<vector<u8>>"),
        txb.pure(response.min_txn_sender, "vector<vector<u8>>"),
        txb.pure(response.min_txn_protocol, "vector<vector<u8>>"),
        txb.pure(response.min_txn_tx_sub_type, "vector<u8>"),
        txb.pure(response.scc_data_hash, "vector<vector<u8>>"),
        txb.pure(response.scc_pair, "vector<vector<u32>>"),
        txb.pure(response.scc_prices, "vector<vector<u128>>"),
        txb.pure(response.scc_timestamp, "vector<vector<u128>>"),
        txb.pure(response.scc_decimals, "vector<vector<u16>>"),
        txb.pure(response.scc_qc, "vector<vector<u8>>"),
        txb.pure(response.scc_round, "vector<u64>"),
        txb.pure(response.scc_id, "vector<vector<u8>>"),
        txb.pure(response.scc_member_index, "vector<u64>"),
        txb.pure(response.scc_committee_index, "vector<u64>"),
        txb.pure(response.batch_idx, "vector<u64>"),
        txb.pure(response.txn_idx, "vector<u64>"),
        txb.pure(response.cluster_idx, "vector<u32>"),
        txb.pure(response.sig, "vector<vector<u8>>"),
        txb.pure(response.pair_mask, "vector<vector<bool>>")
      ]
    });
    ```

# Running the Application

Open your terminal and navigate to the project directory.

Run the example using the following command:

**SUI**

```bash
cargo run --example sui_client
```