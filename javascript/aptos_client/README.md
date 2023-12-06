# Javascript PullServiceClient for Aptos Readme

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

3. Set the chain type Aptos:

   ```bash
   const chainType = 'aptos';
   ```

4. Configure the RPC URL for the desired blockchain network:

   ```bash
   const provider = new aptos.Provider(aptos.Network.LOCAL);
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

3. **Retrieve signer Account**: 
   ```bash
   let account = new aptos.AptosAccount(aptos.HexString.ensure("<PRIVATE KEY>").toUint8Array(), walletAddress);
   ```

4. **Transaction Object**: Customize the transaction object as needed:
   ```bash
   const entryFunctionPayload = new aptos.TxnBuilderTypes.TransactionPayloadEntryFunction(
      aptos.TxnBuilderTypes.EntryFunction.natural(
         `${contractAddress}::${moduleName}`, functionName, [], [
         DkgState,
         OracleHolder,
         response.vote_smr_block_round,
         response.vote_smr_block_timestamp,
         response.vote_smr_block_author,
         response.vote_smr_block_qc_hash,
         response.vote_smr_block_batch_hashes,
         response.vote_round,
         response.min_batch_protocol,
         response.min_batch_txn_hashes,
         response.min_txn_cluster_hashes,
         response.min_txn_sender,
         response.min_txn_protocol,
         response.min_txn_tx_sub_type,
         response.scc_data_hash,
         response.scc_pair,
         response.scc_prices,
         response.scc_timestamp,
         response.scc_decimals,
         response.scc_qc,
         response.scc_round,
         response.scc_id,
         response.scc_member_index,
         response.scc_committee_index,
         response.batch_idx,
         response.txn_idx,
         response.cluster_idx,
         response.sig,
         response.pair_mask
      ]
      ),
   );
   ```

# Running the Application

To run the application, execute the following command

```bash
node main.js
```

This will initiate the fetching of proof data and interaction with the smart contract based on the provided
configuration and customization.