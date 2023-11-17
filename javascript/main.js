const PullServiceClient = require("./pullServiceClient");
const {Web3} = require('web3');

async function main() {
    const address = '<GRPC SERVER ADDRESS>'; // Set the gRPC server address
    const pairIndexes = [0, 21, 61, 49]; // Set the pair indexes as an array
    const chainType = 'evm'; // Set the chain type (evm, sui, aptos)

    const client = new PullServiceClient(address);

    const request = {
        pair_indexes: pairIndexes,
        chain_type: chainType
    };
    console.log("Requesting proof for price index : ", request.pair_indexes);
    client.getProof(request, (err, response) => {
        if (err) {
            console.error('Error:', err.details);
            return;
        }
        console.log("Calling contract to verify the proofs.. ");
        callContract(response.evm)
    });
}

async function callContract(response) {
    const web3 = new Web3(new Web3.providers.HttpProvider('<RPC URL>')); // Rpc url for desired chain

    const contractAbi = require("../resources/abi.json"); // Path of your smart contract ABI


    const contractAddress = '<CONTRACT ADDRESS>'; // Address of your smart contract

    const contract = new web3.eth.Contract(contractAbi, contractAddress);

    const hex = web3.utils.bytesToHex(response.proof_bytes)
    const txData = contract.methods.GetPairPrice(hex, 0).encodeABI(); // function from you contract eg:GetPairPrice from example-contract.sol
    const gasEstimate = await contract.methods.GetPairPrice(hex, 0).estimateGas({from: "<WALLET ADDRESS>"});

    // Create the transaction object
    const transactionObject = {
        from: "<WALLET ADDRESS>",
        to: contractAddress,
        data: txData,
        gas: gasEstimate,
        gasPrice: await web3.eth.getGasPrice() // Set your desired gas price here, e.g: web3.utils.toWei('1000', 'gwei')
    };

    // Sign the transaction with the private key
    const signedTransaction = await web3.eth.accounts.signTransaction(transactionObject, "<PRIVATE KEY>");

    // Send the signed transaction
    const receipt = await web3.eth.sendSignedTransaction(signedTransaction.rawTransaction);
    console.log('Transaction receipt:', receipt);
}

main();