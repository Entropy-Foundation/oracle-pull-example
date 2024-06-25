const { DirectSecp256k1HdWallet } = require('@cosmjs/proto-signing');
const { SigningCosmWasmClient } = require('@cosmjs/cosmwasm-stargate');
const PullServiceClient = require("./pullServiceClient");
async function getWallet(mnemonic) {
    return await DirectSecp256k1HdWallet.fromMnemonic(
        mnemonic,
        {prefix: 'osmo'}  // Use the appropriate prefix for your chain
    );
}
async function getClient(wallet, rpcEndpoint) {
    return await SigningCosmWasmClient.connectWithSigner(
        rpcEndpoint,
        wallet
    );
}

async function executeContract(client, senderAddress, contractAddress, executeMsg, fee) {
    return await client.execute(senderAddress, contractAddress, executeMsg, fee);
}

async function main() {
    const address = '<GRPC_SERVER_ADDRESS>'; // Set the gRPC server address
    const pairIndexes = [0]; // Set the pair indexes as an array
    const chainType = 'cosmwasm'; // Set the chain type (evm, sui, aptos)

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
        callContract(response.cosmwasm)
    });
}

async function callContract(response) {
    const mnemonic = "<MNEMONIC KEY>";
    const rpcEndpoint = "<RPC_ENDPOINT>";
    const wallet = await getWallet(mnemonic);
    const client = await getClient(wallet, rpcEndpoint);

    const contract_address = "<CONTRACT_ADDRESS>";

    const execute_msg = {
        verify_oracle_proof: {
            bytes_proof : Array.from(response.proof_bytes)
        }
    };

    const fee = { amount: [{ denom: "uosmo", amount: "5000" }], gas: "1000000" };
    const sender_address = "<SENDER_ADDRESS>";
    const execute_result = await executeContract(client,sender_address,contract_address,execute_msg,fee);
    console.log("Execute result:", execute_result);

}

main();