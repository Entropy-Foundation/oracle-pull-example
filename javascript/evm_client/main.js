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

    const contractAbi = require("../../resources/abi.json"); // Path of your smart contract ABI

    const contractAddress = '<CONTRACT ADDRESS>'; // Address of your smart contract

    const contract = new web3.eth.Contract(contractAbi, contractAddress);

    const hex = web3.utils.bytesToHex(response.proof_bytes);

    /////////////////////////////////////////////////// Utility code to deserialise the oracle proof bytes ///////////////////////////////////////////////////////////////////

    const OracleProofABI = require("../../resources/oracleProof..json"); // Interface for the Oracle Proof data

    const SignedCoherentClusterABI = require("../../resources/signedCoherentCluster.json");  // Interface for the Signed pair cluster data

    let proof_data = web3.eth.abi.decodeParameters(OracleProofABI,hex); // Deserialising the Oracle Proof data 

    let clusters=proof_data[0].clustersRaw; // Fatching the raw bytes of the signed pair cluster data
    let pairMask=proof_data[0].pairMask; // Fetching which pair ids is been requested
    let pair = 0;  // Helps in iterating the vector of pair masking
    let pairId=[]  // list of all the pair ids requested
    let pairPrice=[]; // list of prices for the corresponding pair ids  
    let pairDecimal=[]; // list of pair decimals for the corresponding pair ids
    let pairTimestamp=[]; // list of pair last updated timestamp for the corresponding pair ids


    for (let i = 0; i < clusters.length; ++i) {


      let scc = web3.eth.abi.decodeParameters(SignedCoherentClusterABI,clusters[i]); // deserialising the raw bytes of the signed pair cluster data
      
      for (let j = 0; j < scc[0].cc.pair.length; ++j) {
          pair += 1;
          if (!pairMask[pair - 1]) { // verifying whether the pair is requested or not
              continue;
          }
          pairId.push(scc[0].cc.pair[j].toString(10)); // pushing the pair ids requested in the output vector

          pairPrice.push(scc[0].cc.prices[j].toString(10)); // pushing the pair price for the corresponding ids 

          pairDecimal.push(scc[0].cc.decimals[j].toString(10)); // pushing the pair decimals for the corresponding ids requested

          pairTimestamp.push(scc[0].cc.timestamp[j].toString(10)); // pushing the pair timestamp for the corresponding ids requested


      }
    }

    console.log("Pair index : ", pairId);
    console.log("Pair Price : ", pairPrice);
    console.log("Pair Decimal : ", pairDecimal);
    console.log("Pair Timestamp : ", pairTimestamp);


    /////////////////////////////////////////////////// End of the utility code to deserialise the oracle proof bytes ////////////////////////////////////////////////////////////////
    
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