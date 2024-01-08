const aptos = require("aptos");
const provider = new aptos.Provider({ fullnodeUrl: "<RPC URL>" });
const PullServiceClient = require("./pullServiceClient");

async function main() {
    const address = '<GRPC SERVER ADDRESS>'; // Set the gRPC server address
    const pairIndexes = [0, 21, 61, 49]; // Set the pair indexes as an array
    const chainType = 'aptos';

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
        callContract(response.aptos)
    });
}

async function callContract(response) {

    const contractAddress = '<CONTRACT ADDRESS>'; // Address of your smart contract
    const walletAddress = '<WALLET ADDRESS>'; // wallet address of caller
    const moduleName = "<CONTRACT MODULE>"; // Module name of your contract. Ex. MockOracleClient
    const functionName = "<CONTRACT FUNCTION>"; // Module function name of your contract.

    const DkgState = aptos.BCS.bcsToBytes(aptos.TxnBuilderTypes.AccountAddress.fromHex(response.dkg_object));
    const OracleHolder = aptos.BCS.bcsToBytes(aptos.TxnBuilderTypes.AccountAddress.fromHex(response.oracle_holder_object))

    let account = new aptos.AptosAccount(aptos.HexString.ensure("<PRIVATE KEY>").toUint8Array(), walletAddress);

    const entryFunctionPayload = new aptos.TxnBuilderTypes.TransactionPayloadEntryFunction(
        aptos.TxnBuilderTypes.EntryFunction.natural(
            `${contractAddress}::${moduleName}`, functionName, [], [
            DkgState,
            OracleHolder,
            response.vote_smr_block_round,
            response.vote_smr_block_timestamp,
            response.vote_smr_block_author,
            response.vote_smr_block_qc_hash,
            response.vote_smr_block_batch_hashes,
            response.vote_round,
            response.min_batch_protocol,
            response.min_batch_txn_hashes,
            response.min_txn_cluster_hashes,
            response.min_txn_sender,
            response.min_txn_protocol,
            response.min_txn_tx_sub_type,
            response.scc_data_hash,
            response.scc_pair,
            response.scc_prices,
            response.scc_timestamp,
            response.scc_decimals,
            response.scc_qc,
            response.scc_round,
            response.scc_id,
            response.scc_member_index,
            response.scc_committee_index,
            response.batch_idx,
            response.txn_idx,
            response.cluster_idx,
            response.sig,
            response.pair_mask
        ]
        ),
    );

    const rawTxn = await provider.generateRawTransaction(account.address(), entryFunctionPayload);

    let result = await provider.signAndSubmitTransaction(account, rawTxn);
    console.log("Transaction hash: ", result);
}

main();