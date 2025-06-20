# Javascript PullServiceClient for SUI Readme

This library is designed to interact with a rest api server for fetching proof data and then using that data to call a smart
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

The library provides the main function, which fetches proof data from the rest api server using the specified parameters and
then calls a contract function on a blockchain network.

# Configuration

Before using the library, make sure to set up the configuration in the main.js file:

1. Set the rest api server address:

   ```js
   const address = 'https://rpc-testnet-dora-2.supra.com/';
   ```
2. Set the pair indexes as an array:

   ```js
   const pairIndexes = [0, 21, 61, 49];
   ```

3. Set the chain type sui:

   ```js
   const chainType = 'sui';
   ```

4. Configure the RPC URL for the desired blockchain network:

   ```js
   const rpcUrl = suiSdk.getFullnodeUrl('testnet');
   const suiClient = new suiSdk.SuiClient({ url: rpcUrl });
   ```

# Customization

Users can customize the smart contract interaction under the callContract function. Specifically, you can modify the
following components:

1. **Smart Contract Address**: Set the address of your smart contract:
   ```js
   const contractAddress = '<CONTRACT ADDRESS>';
   ```

2. **Function Call**: Modify the function call according to your smart contract's methods. For example, if your smart contract has a module named `pull_example` & method named `get_pair_price`:
   ```js
   const moduleName = "pull_example";
   const functionName = "get_pair_price";
   ```

3. **Transaction Object**: Customize the transaction object as needed:
   ```js
   txb.moveCall({ 
       target: `${contractAddress}::${moduleName}::${functionName}`,
       arguments: [
         txb.object(response.dkg_object),
         txb.object(response.oracle_holder_object),
         txb.object(response.merkle_root_object),
         txb.object(CLOCK),
         txb.pure(bytes, "vector<u8>"),
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