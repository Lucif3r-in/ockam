# Credentials

This is an example of how to use the credentials module.

## Issuing Credentials

To issue credentials, you can use the `CredentialsIssuer` struct. Here is an example:

```rust
use ockam::identity::{CredentialsIssuer, Vault};
use ockam::Context;

#[ockam::node]
async fn main(ctx: Context) -> Result<(), Box<dyn std::error::Error>> {
    // Create a Vault
    let vault = Vault::create(ctx)?;

    // Create a CredentialsIssuer
    let issuer = CredentialsIssuer::new(vault)?;

    // Issue a credential
    let credential = issuer.issue_credential("Alice", "Bob").await?;

    // Print the issued credential
    println!("Issued credential: {:?}", credential);

    Ok(())
}
```

This example demonstrates how to create a `CredentialsIssuer` using a `Vault` and issue a credential to a subject named "Alice" with a verifier named "Bob".

## Verifying Credentials

To verify credentials, you can use the `CredentialsVerifier` struct. Here is an example:

```rust
use ockam::identity::{CredentialsVerifier, Vault};
use ockam::Context;

#[ockam::node]
async fn main(ctx: Context) -> Result<(), Box<dyn std::error::Error>> {
    // Create a Vault
    let vault = Vault::create(ctx)?;

    // Create a CredentialsVerifier
    let verifier = CredentialsVerifier::new(vault)?;

    // Verify a credential
    let credential = get_credential_from_somewhere();

    let result = verifier.verify_credential(&credential).await?;

    if result {
        println!("Credential is valid");
    } else {
        println!("Credential is invalid");
    }

    Ok(())
}
```

This example demonstrates how to create a `CredentialsVerifier` using a `Vault` and verify a credential obtained from somewhere.

Please note that the above examples are simplified for demonstration purposes and may not cover all edge cases. Make sure to adapt them to your specific use case.
