mod spooler_impl;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let spooler_impl = spooler_impl::SpoolerService::new();

    spooler_impl.run().await?;

    Ok(())
}
