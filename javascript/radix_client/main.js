const PullServiceClient = require("./pullServiceClient");
const GATEWAY_URL  = "https://stokenet.radixdlt.com";
const NETWORK_ID = 2;

const NetworkConfiguration = {
    gatewayBaseUrl: GATEWAY_URL,
    networkId: NETWORK_ID,
};

const {
    Configuration,
    StatusApi,
    TransactionApi,
    TransactionStatus,
} = require ("@radixdlt/babylon-gateway-api-sdk");
const {
    Convert,
    ManifestBuilder,
    PrivateKey,
    RadixEngineToolkit,
    TransactionBuilder,
    decimal,
    generateRandomNonce,
    blob,
    hash,
    nonFungibleLocalId,
    array,
    ValueKind,
} = require ("@radixdlt/radix-engine-toolkit");

async function main() {
    const address = '<GRPC_SERVER>'; // Set the gRPC server address
    const pairIndexes = [0, 21, 61, 49]; // Set the pair indexes as an array
    const chainType = 'radix'; // Set the chain type (evm, sui, aptos, radix)

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
        invokeRadixChain(response.radix)
    });
}

const getCurrentEpoch = async (statusApi) =>
    statusApi.gatewayStatus().then((output) => output.ledger_state.epoch);

const submitTransaction = async (
    transactionApi,
    compiledTransaction
) =>
    transactionApi.transactionSubmit({
        transactionSubmitRequest: {
            notarized_transaction_hex:
                Convert.Uint8Array.toHexString(compiledTransaction),
        },
    });

const getTransactionStatus = async (
    transactionApi,
    transactionId
) => transactionApi.transactionStatus({
        transactionStatusRequest: {
            intent_hash: transactionId,
        },
    });


const invokeRadixChain = async (response) => {
    const apiConfiguration = new Configuration({
        basePath: NetworkConfiguration.gatewayBaseUrl,
    });
    const statusApi = new StatusApi(apiConfiguration);
    const transactionApi = new TransactionApi(apiConfiguration);

    // Setting up the private key.
    const notaryPrivateKey = new PrivateKey.Ed25519(
        "<PRIVATE_KEY>"
    );

    // Building the manifest of this example. The manifest for this example will be quite simple: it
    // will lock some amount of XRD in fees from the faucet's component.
    const faucetComponentAddress = await RadixEngineToolkit.Utils.knownAddresses(
        NetworkConfiguration.networkId
    ).then((addressBook) => addressBook.componentAddresses.faucet);

    let hex_payload_bytes = Convert.HexString.toUint8Array(response.proof_bytes.toString("hex"));

    const manifest = new ManifestBuilder()
        .callMethod(faucetComponentAddress, "lock_fee", [decimal(5000)])
        .callMethod(
            "<COMPONENT_ADDRESS>",
            "<COMPONENT METHOD>",
            [
                blob(hash(hex_payload_bytes)),
                array(ValueKind.NonFungibleLocalId,nonFungibleLocalId("{<NONFUNGIBLE-RUID>}"))
            ]
        )
        .build();
        manifest.blobs.push(response.proof_bytes);

    // Generating Tx
    const currentEpoch = await getCurrentEpoch(statusApi);
    const notarizedTransaction = await TransactionBuilder.new().then((builder) =>
        builder
            .header({
                networkId: NetworkConfiguration.networkId,
                startEpochInclusive: currentEpoch,
                endEpochExclusive: currentEpoch + 10,
                nonce: generateRandomNonce(),
                notaryPublicKey: notaryPrivateKey.publicKey(),
                notaryIsSignatory: false,
                tipPercentage: 0,
            })
            .manifest(manifest)
            .notarize(notaryPrivateKey)
    );
    // Generating tx id.
    const transactionId =
        await RadixEngineToolkit.NotarizedTransaction.intentHash(
            notarizedTransaction
        );
    console.log("Transaction ID:", transactionId);

    // Tx details.
    console.log("Transaction:", notarizedTransaction);

    // Generating tx bytes by compiling it
    const compiledTransaction =
        await RadixEngineToolkit.NotarizedTransaction.compile(notarizedTransaction);
    console.log(
        "Compiled Transaction:",
        Convert.Uint8Array.toHexString(compiledTransaction)
    );

    console.log("Submitting the tx");
    const submissionResult = await submitTransaction(
        transactionApi,
        compiledTransaction
    );
    console.log("Transaction submission result:", submissionResult);

    // Waiting for tx to committed
    let transactionStatus = undefined;
    while (
        transactionStatus === undefined ||
        transactionStatus?.status === TransactionStatus.Pending
        ) {
        transactionStatus = await getTransactionStatus(
            transactionApi,
            transactionId.id
        );
        await new Promise((r) => setTimeout(r, 1000));
    }
    console.log("Transaction Status:", transactionStatus);
};

main();