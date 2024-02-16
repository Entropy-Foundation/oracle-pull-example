# Javascript PullServiceClient for EVM Readme

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

3. Set the chain type evm:

   ```js
   const chainType = 'evm';
   ```

4. Configure the RPC URL for the desired blockchain network:

   ```js
   const web3 = new Web3(new Web3.providers.HttpProvider('<RPC URL>'));
   ```

# Customization

Users can customize the smart contract interaction under the callContract function. Specifically, you can modify the
following components:

1. **Smart Contract ABI**: Update the path to your smart contract's ABI JSON file:
   ```js
   const contractAbi = require("../resources/abi.json");
   ```

2. **Smart Contract Address**: Set the address of your smart contract:

   ```js
   const contractAddress = '<CONTRACT ADDRESS>';
   ```

3. **Function Call**: Modify the function call according to your smart contract's methods. For example, if your smart
   contract has a method named GetPairPrice:
   ```js
   const txData = contract.methods.GetPairPrice(hex, 0).encodeABI();
   ```

4. **Gas Estimate**: Adjust the gas estimation by calling the desired contract method:
   ```js
   const gasEstimate = await contract.methods.GetPairPrice(hex, 0).estimateGas({ from: "<WALLET ADDRESS>" });
   ```

5. **Transaction Object**: Customize the transaction object as needed:
   ```js
   const transactionObject = {
    from: "<WALLET ADDRESS>",
    to: contractAddress,
    data: txData,
    gas: gasEstimate,
    gasPrice: await web3.eth.getGasPrice() // Set your desired gas price here, e.g: web3.utils.toWei('1000', 'gwei')
   };
   ```

6. **Private Key Signing**: Sign the transaction with the appropriate private key:
   ```js
   const signedTransaction = await web3.eth.accounts.signTransaction(transactionObject, "<PRIVATE KEY>");
   ```

# Running the Application

To run the application, execute the following command

```bash
node main.js
```

This will initiate the fetching of proof data and interaction with the smart contract based on the provided
configuration and customization.