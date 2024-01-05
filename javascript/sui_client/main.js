const suiSdk = require("@mysten/sui.js/client");
const rpcUrl = suiSdk.getFullnodeUrl('localnet');
const suiClient = new suiSdk.SuiClient({ url: rpcUrl });

const suiTx = require("@mysten/sui.js/transactions");
const suiKeypair = require("@mysten/sui.js/keypairs/ed25519");
const suiUtils = require("@mysten/sui.js/utils");

const PullServiceClient = require("./pullServiceClient");

async function main() {
    const address = '<GRPC SERVER ADDRESS>'; // Set the gRPC server address
    const pairIndexes = [0, 21, 61, 49]; // Set the pair indexes as an array
    const chainType = 'sui';

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
        callContract(response.sui)
    });
}

async function callContract(response) {

    const contractAddress = '<CONTRACT ADDRESS>'; // Address of your smart contract
    const moduleName = "<CONTRACT MODULE>"; // Module name of your contract. Ex. pull_example
    const functionName = "<CONTRACT FUNCTION>"; // Module function name of your contract. Ex. get_pair_price

    let txb = new suiTx.TransactionBlock();

    txb.moveCall({
        target: `${contractAddress}::${moduleName}::${functionName}`,
        arguments: [
            txb.pure(response.dkg_object),
            txb.pure(response.oracle_holder_object),

            txb.pure(response.vote_smr_block_round, "vector<vector<u8>>"),
            txb.pure(response.vote_smr_block_timestamp, "vector<vector<u8>>"),
            txb.pure(response.vote_smr_block_author, "vector<vector<u8>>"),
            txb.pure(response.vote_smr_block_qc_hash, "vector<vector<u8>>"),
            txb.pure(response.vote_smr_block_batch_hashes, "vector<vector<u8>>"),
            txb.pure(response.vote_round, "vector<u64>"),

            txb.pure(response.min_batch_protocol, "vector<vector<u8>>"),
            txb.pure(response.min_batch_txn_hashes, "vector<vector<vector<u8>>>"),

            txb.pure(response.min_txn_cluster_hashes, "vector<vector<u8>>"),
            txb.pure(response.min_txn_sender, "vector<vector<u8>>"),
            txb.pure(response.min_txn_protocol, "vector<vector<u8>>"),
            txb.pure(response.min_txn_tx_sub_type, "vector<u8>"),

            txb.pure(response.scc_data_hash, "vector<vector<u8>>"),
            txb.pure(response.scc_pair, "vector<vector<u32>>"),
            txb.pure(response.scc_prices, "vector<vector<u128>>"),
            txb.pure(response.scc_timestamp, "vector<vector<u128>>"),
            txb.pure(response.scc_decimals, "vector<vector<u16>>"),
            txb.pure(response.scc_qc, "vector<vector<u8>>"),
            txb.pure(response.scc_round, "vector<u64>"),
            txb.pure(response.scc_id, "vector<vector<u8>>"),
            txb.pure(response.scc_member_index, "vector<u64>"),
            txb.pure(response.scc_committee_index, "vector<u64>"),

            txb.pure(response.batch_idx, "vector<u64>"),
            txb.pure(response.txn_idx, "vector<u64>"),
            txb.pure(response.cluster_idx, "vector<u32>"),
            txb.pure(response.sig, "vector<vector<u8>>"),
            txb.pure(response.pair_mask, "vector<vector<bool>>")
        ]
    });

    const raw = suiUtils.fromB64("<PRIVATE KEY BASE64>"); // Your wallet private in base64 format
    let signer = suiKeypair.Ed25519Keypair.fromSecretKey(raw.slice(1));

    const result = await suiClient.signAndExecuteTransactionBlock({ transactionBlock: txb, signer });
    console.log({ result });
}

main();
