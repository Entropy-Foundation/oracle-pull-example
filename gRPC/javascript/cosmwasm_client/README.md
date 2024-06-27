# Javascript PullServiceClient for Cosmwasm Readme

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

   ```js
   const address = 'testnet-dora-2.supra.com';
   ```
2. Set the pair indexes as an array:

   ```js
   const pairIndexes = [0, 21, 61, 49];
   ```

3. Set the chain type Cosmwasm:

   ```js
   const chainType = 'cosmwasm';
   ```

4. Configure the RPC URL for the desired blockchain network:

   ```js
   const rpcEndpoint = "<RPC_ENDPOINT>";
   ```
   
5. Configure the wallet by setting the mnemonic, which combined with the RPC will be used to setup the Cosmwasm client
   ```js
   const mnemonic = "<MNEMONIC KEY>";
   ```
# Customization

Users can customize the smart contract interaction under the callContract function. Specifically, you can modify the
following components:

1. **Smart Contract Address**: Set the address of your smart contract:
   ```js
   const contractAddress = '<CONTRACT ADDRESS>';
   ```

2. **Signer/Sender Address**: Modify the Sender wallet address, basically the wallet address through which the transaction is to be initiated:
   ```js
   const sender_address = "<SENDER_ADDRESS>";
   ```

# Running the Application

To run the application, execute the following command

```bash
node main.js
```

This will initiate the fetching of proof data and interaction with the smart contract based on the provided
configuration and customization.