# Javascript PullServiceClient for SUI Readme

This library is designed to interact with a gRPC server for fetching proof data and then using that data to call a smart
contract on a blockchain network. This readme provides instructions on how to use the library and customize certain
components for your specific use case.

## Installation

To use the PullServiceClient library, follow these steps:

1. Clone the repository or download the library's source code.
2. Install the necessary dependencies by running the following command in your project directory:

   ```bash
   npm install
   ```

# Usage

The library provides the main function, which fetches proof data from the gRPC server using the specified parameters and
then calls a contract function on a blockchain network.

# Configuration

Before using the library, make sure to set up the configuration in the main.js file:

1. Set the gRPC server address:

   ```bash
   const address = 'testnet-dora.supraoracles.com';
   ```
2. Set the pair indexes as an array:

   ```bash
   const pairIndexes = [0, 21, 61, 49];
   ```

3. Set the chain type sui:

   ```bash
   const chainType = 'sui';
   ```

4. Configure the RPC URL for the desired blockchain network:

   ```bash
   const rpcUrl = suiSdk.getFullnodeUrl('testnet');
   const suiClient = new suiSdk.SuiClient({ url: rpcUrl });
   ```

# Customization

Users can customize the smart contract interaction under the callContract function. Specifically, you can modify the
following components:

1. **Smart Contract Address**: Set the address of your smart contract:
   ```bash
   const contractAddress = '<CONTRACT ADDRESS>';
   ```

2. **Function Call**: Modify the function call according to your smart contract's methods. For example, if your smart contract has a module named `pull_example` & method named `get_pair_price`:
   ```bash
   const moduleName = "pull_example";
   const functionName = "get_pair_price";
   ```

3. **Transaction Object**: Customize the transaction object as needed:
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

4. **Private Key Signing**: Sign the transaction with the appropriate private key:
   ```bash
   const raw = suiUtils.fromB64("<PRIVATE KEY BASE64>");
   ```

# Running the Application

To run the application, execute the following command

```bash
node main.js
```

This will initiate the fetching of proof data and interaction with the smart contract based on the provided
configuration and customization.