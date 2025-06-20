import { SupraClient, SupraAccount, BCS } from "supra-l1-sdk";
import PullServiceClient from "./pullServiceClient.js";

let supra_client = await SupraClient.init("<RPC ENDPOINT>");

async function main() {
    const address = '<REST API SERVER ADDRESS>'; // Set the REST API server address
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
            console.error('Error:', error ? .response ? .data);
        });
}

async function callContract(response) {

    const contractAddress = '<CONTRACT ADDRESS>'; // Address of your smart contract
    const privateKey = '<WALLET PRIVATE KEY>'; // wallet address of caller
    const moduleName = "<CONTRACT MODULE>"; // Module name of your contract. Ex. MockOracleClient
    const functionName = "<CONTRACT FUNCTION>"; // Module function name of your contract.

    const priv_key_bytes = Uint8Array.from(Buffer.from(privateKey, "hex"));
    const account = new SupraAccount(priv_key_bytes);

    console.log("Account address:", account.address());

    let supraRawTransaction = await supra_client.createRawTxObject(
        account.address(),
        (
            await supra_client.getAccountInfo(account.address())
        ).sequence_number,
        contractAddress,
        moduleName,
        functionName, [], [
            BCS.bcsSerializeBytes(response.proof_bytes)
        ]
    );

    let supraTransferRawTransactionSerializer = new BCS.Serializer();
    supraRawTransaction.serialize(
        supraTransferRawTransactionSerializer
    );
    console.log(
        await supra_client.sendTxUsingSerializedRawTransaction(
            account,
            supraTransferRawTransactionSerializer.getBytes(), {
                enableWaitForTransaction: true,
                enableTransactionSimulation: true,
            }
        )
    );
}

main();