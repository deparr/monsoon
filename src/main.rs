use monsoon::*;

// TODO do tmp file -- done?
// Daemonize?
// Signal handlers?

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    return run().await;
}

