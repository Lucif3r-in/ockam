// Updated code block from 04-secure-channel-over-two-transport-hops-initiator.rs

use ockam::{Context, Result, SecureChannel, TcpTransport, Vault};
use ockam_transport_tcp::TcpTransport;
use ockam_vault_sync_core::Vault;

#[ockam::node]
async fn main(mut ctx: Context) -> Result<()> {
    // Create a TCP transport
    let tcp = TcpTransport::create(&ctx).await?;

    // Create a secure channel
    let vault = Vault::create(&ctx).await?;
    let secure_channel = SecureChannel::create(&ctx, vault)?;

    // Create a TCP connection
    let connection = tcp.connect("127.0.0.1:4000").await?;

    // Create a secure channel connection
    let channel = secure_channel.connect(&connection).await?;

    // Send a message over the secure channel
    channel.send("Hello Ockam!".into()).await?;

    // Receive a message over the secure channel
    let message = channel.receive::<String>().await?;
    println!("Received: {}", message);

    Ok(())
}
