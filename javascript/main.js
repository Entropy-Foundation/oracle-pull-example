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

    const OracleProofABI = [
      {
        type:'tuple',
        name:'OracleProof',
        components:[
          {
            type: 'tuple[]',
            name: 'votes',
            components: [
              // Components of Vote struct
              {
                type: 'tuple',
                name: 'smrBlock',
                components: [
                  { type: 'uint64', name: 'round' },
                  { type: 'uint128', name: 'timestamp' },
                  { type: 'bytes32', name: 'author' },
                  { type: 'bytes32', name: 'qcHash' },
                  { type: 'bytes32[]', name: 'batchHashes' }
                ]
              },
              { type: 'bytes8', name: 'roundLE' }
            ]
          },
          {
            type: 'uint256[2][]',
            name: 'sigs'
          },
          {
            type: 'tuple[]',
            name: 'smrBatches',
            components: [
              // Components of MinBatch struct
              { type: 'bytes10', name: 'protocol' },
              { type: 'bytes32[]', name: 'txnHashes' },
              { type: 'uint256', name: 'batchIdx' }
            ]
          },
          {
            type: 'tuple[]',
            name: 'smrTxns',
            components: [
              // Components of MinTxn struct
              { type: 'bytes32[]', name: 'clusterHashes' },
              { type: 'bytes32', name: 'sender' },
              { type: 'bytes10', name: 'protocol' },
              { type: 'bytes1', name: 'tx_sub_type' },
              { type: 'uint256', name: 'txnIdx' }
            ]
          },
          {
            type: 'bytes[]',
            name: 'clustersRaw'
          },
          {
            type: 'uint256[]',
            name: 'batchToVote'
          },
          {
            type: 'uint256[]',
            name: 'txnToBatch'
          },
          {
            type: 'uint256[]',
            name: 'clusterToTxn'
          },
          {
            type: 'uint256[]',
            name: 'clusterToHash'
          },
          {
            type: 'bool[]',
            name: 'pairMask'
          },
          {
            type: 'uint256',
            name: 'pairCnt'
          }
        ]
      }
    ];

    const SignedCoherentClusterABI = [
      {
        type: 'tuple',
        name: 'scc',
        components :[
          {
            type: 'tuple',
            name: 'cc',
            components: [
              { type: 'bytes32', name: 'dataHash' },
              { type: 'uint256[]', name: 'pair' },
              { type: 'uint256[]', name: 'prices' },
              { type: 'uint256[]', name: 'timestamp' },
              { type: 'uint256[]', name: 'decimals' }
            ]
          },
          { type: 'bytes', name: 'qc' },
          { type: 'uint256', name: 'round' },
          {
            type: 'tuple',
            name: 'origin',
            components: [
              { type: 'bytes32', name: '_publicKeyIdentity' },
              { type: 'uint256', name: '_pubMemberIndex' },
              { type: 'uint256', name: '_committeeIndex' }
            ]
          }

        ]
      }
      
    ];

    const web3 = new Web3(new Web3.providers.HttpProvider('<RPC URL>')); // Rpc url for desired chain

    const contractAbi = require("../resources/abi.json"); // Path of your smart contract ABI


    const contractAddress = '<CONTRACT ADDRESS>'; // Address of your smart contract

    const contract = new web3.eth.Contract(contractAbi, contractAddress);

    const hex = web3.utils.bytesToHex(response.proof_bytes)
    let proof_data = web3.eth.abi.decodeParameters(OracleProofABI,hex)

    let clusters=proof_data[0].clustersRaw;
    let pairMask=proof_data[0].pairMask;
    let pairId=[]
    let pairPrice=[];
    let pairDecimal=[];
    let pairTimestamp=[];


    for (let i = 0; i < clusters.length; ++i) {


      let scc = web3.eth.abi.decodeParameters(SignedCoherentClusterABI,clusters[i]);
      let pair = 0; 
      for (let j = 0; j < scc[0].cc.pair.length; ++j) {
          pair += 1;
          if (!pairMask[pair - 1]) {
              continue;
          }
          pairId.push(scc[0].cc.pair[j].toString(10));

          pairPrice.push(scc[0].cc.prices[j].toString(10));

          pairDecimal.push(scc[0].cc.decimals[j].toString(10));

          pairTimestamp.push(scc[0].cc.timestamp[j].toString(10));


      }
    }

    console.log("Pair index : ", pairId);
    console.log("Pair Price : ", pairPrice);
    console.log("Pair Decimal : ", pairDecimal);
    console.log("Pair Timestamp : ", pairTimestamp);
    
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