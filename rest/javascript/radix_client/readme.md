# Javascript PullServiceClient for Radix Readme

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
   const chainType = 'radix';
   ```

4. Configure the Network Detail for the desired radix network eg: Stokenet:

   ```js
    const GATEWAY_URL  = "https://stokenet.radixdlt.com";
    const NETWORK_ID = 2;
   ```

# Customization

Users can customize the smart contract interaction under the callContract function. Specifically, you can modify the
following components:

1. **Private Key**: Update <PRIVATE_KEY> with you hex encoded ed25519 secret key:
   ```js
   const notaryPrivateKey = new PrivateKey.Ed25519(
        "<PRIVATE_KEY>"
   );
   ```

2. **Component Details**: Set your component address and function name along with parameters:

   ```js
   const manifest = new ManifestBuilder()
        .callMethod(faucetComponentAddress, "lock_fee", [decimal(5000)])
        .callMethod(
            "<COMPONENT_ADDRESS>",
            "<COMPONENT METHOD>",
            [
                blob(hash(hex_payload_bytes))
            ]
        )
        .build();
        manifest.blobs.push(response.proof_bytes);
   ```

# Running the Application

To run the application, execute the following command

```bash
node main.js
```

This will initiate the fetching of proof data and interaction with the smart contract based on the provided
configuration and customization.