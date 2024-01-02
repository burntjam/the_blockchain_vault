fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("../spool_server/proto/spool.proto")?;
    Ok(())
}
