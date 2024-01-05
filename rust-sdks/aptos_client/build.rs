fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("../../protos/client.proto")?; //Path to proto file
    Ok(())
}
