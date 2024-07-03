const suiSdk = require("@mysten/sui.js/client");
const rpcUrl = suiSdk.getFullnodeUrl('testnet');
const suiClient = new suiSdk.SuiClient({ url: rpcUrl });

const suiTx = require("@mysten/sui.js/transactions");
const suiKeypair = require("@mysten/sui.js/keypairs/ed25519");
const suiUtils = require("@mysten/sui.js/utils");

const suiBsc = require("@mysten/sui.js/bcs");

const PullServiceClient = require("./pullServiceClient");
const CLOCK = "0x6";
async function main() {
    const address = '<REST API SERVER ADDRESS>'; // Set the REST server address
    const pairIndexes = [0, 21, 61, 49]; // Set the pair indexes as an array
    const chainType = 'sui'; // Set the chain type (evm, sui, aptos, radix)

    const client = new PullServiceClient(address);

    const request = {
        pair_indexes: pairIndexes,
        chain_type: chainType
    };
    console.log("Requesting proof for price index : ", request.pair_indexes);
    client.getProof(request)
        .then(response => {
            console.log('Proof received:', response);
            callContract(response)
        })
        .catch(error => {
            console.error('Error:', error?.response?.data);
        });
}

async function callContract(response) {

    const contractAddress = '<CONTRACT ADDRESS>'; // Address of your smart contract
    const moduleName = "<CONTRACT MODULE>"; // Module name of your contract. Ex. pull_example
    const functionName = "<CONTRACT FUNCTION>"; // Module function name of your contract. Ex. get_pair_price

    let txb = new suiTx.TransactionBlock();

    let buffer = Buffer.from(response.proof_bytes, 'hex');

    txb.moveCall({
        target: `${contractAddress}::${moduleName}::${functionName}`,
        arguments: [
            txb.object(response.dkg_object),
            txb.object(response.oracle_holder_object),
            txb.object(response.merkle_root_object),
            txb.object(CLOCK),
            txb.pure(Array.from(buffer), "vector<u8>"),
        ]
    });

    const raw = suiUtils.fromB64("<PRIVATE KEY BASE64>"); // Your wallet private in base64 format
    let signer = suiKeypair.Ed25519Keypair.fromSecretKey(raw.slice(1));

    const result = await suiClient.signAndExecuteTransactionBlock({ transactionBlock: txb, signer });
    console.log({ result });
}

main();
