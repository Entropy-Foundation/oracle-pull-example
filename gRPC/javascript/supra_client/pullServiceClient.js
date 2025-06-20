import grpc from '@grpc/grpc-js';
import protoLoader from '@grpc/proto-loader';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(
    import.meta.url);
const __dirname = path.dirname(__filename);

class PullServiceClient {
    constructor(address) {
        // Path to the protobuf definition file
        var PROTO_PATH = __dirname + './../../protos/client.proto';

        const packageDefinition = protoLoader.loadSync(PROTO_PATH, {
            keepCase: true,
            longs: String,
            enums: String,
            defaults: true,
            oneofs: true,
        });

        // Load the gRPC package definition
        const pullProto = grpc.loadPackageDefinition(packageDefinition).pull_service;

        // Create gRPC client with SSL credentials
        this.client = new pullProto.PullService(address, grpc.credentials.createSsl());
    }

    getProof(request, callback) {
        this.client.getProof(request, callback);
    }
}

export default PullServiceClient;