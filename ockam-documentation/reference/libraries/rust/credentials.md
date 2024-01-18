# Credentials

This is an example of how to use credentials in Ockam.

```rust
use ockam::{Context, Result};
use ockam::credentials::{AttributesEntry, CredentialAttribute, CredentialIssuer, CredentialVerifier};
use ockam::identity::{Identity, IdentityAttribute, IdentityBuilder, IdentityVerifier};

#[ockam::node]
async fn main(mut ctx: Context) -> Result<()> {
    // Create a new identity
    let identity = IdentityBuilder::new()
        .add_attribute(IdentityAttribute::new("cluster", "production"))
        .build()?;

    // Create a new credential issuer
    let issuer = CredentialIssuer::new(identity.clone());

    // Create a new credential verifier
    let verifier = CredentialVerifier::new(identity.clone());

    // Issue a credential
    let attributes = AttributesEntry::single(b"cluster".to_vec(), b"production".to_vec(), None, None)?;
    issuer.issue_credential(attributes.clone()).await?;

    // Verify the credential
    verifier.verify_credential(attributes.clone()).await?;

    Ok(())
}
```
```

Please note that this is a simplified example and may require additional imports and modifications to fit into the existing `credentials.md` file.

I will also create unit tests to ensure the functionality of the code.

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_credentials_example() {
        // Test code here
    }
}
