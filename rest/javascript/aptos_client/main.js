const aptos = require("aptos");
const provider = new aptos.Provider(aptos.Network.TESTNET);
const PullServiceClient = require("./pullServiceClient");

async function main() {
    const address = '<REST API SERVER ADDRESS>'; // Set the rest server address
    const pairIndexes = [0, 21, 61, 49]; // Set the pair indexes as an array
    const chainType = 'aptos'; // Set the chain type (evm, sui, aptos, radix)

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
    const walletAddress = '<WALLET ADDRESS>'; // wallet address of caller
    const moduleName = "<CONTRACT MODULE>"; // Module name of your contract. Ex. MockOracleClient
    const functionName = "<CONTRACT FUNCTION>"; // Module function name of your contract.

    const OracleHolder = aptos.BCS.bcsToBytes(aptos.TxnBuilderTypes.AccountAddress.fromHex(response.oracle_holder_object))

    let account = new aptos.AptosAccount(aptos.HexString.ensure("<PRIVATE KEY>").toUint8Array(), walletAddress);

    const entryFunctionPayload = new aptos.TxnBuilderTypes.TransactionPayloadEntryFunction(
        aptos.TxnBuilderTypes.EntryFunction.natural(
            `${contractAddress}::${moduleName}`, functionName, [], [
                OracleHolder,
                aptos.BCS.bcsSerializeBytes(response.proof_bytes)
            ]
        ),
    );

    const rawTxn = await provider.generateRawTransaction(account.address(), entryFunctionPayload);

    let result = await provider.signAndSubmitTransaction(account, rawTxn);
    console.log("Transaction hash: ", result);
}

main();