// ockam-documentation/reference/libraries/rust/credentials.md

// Import necessary files
use ockam::credentials::{AttributesEntry, CredentialIssuer, CredentialVerifier, SubjectAttributes};
use ockam::vault::SoftwareVault;

fn main() {
    // Create a new software vault
    let vault = SoftwareVault::default();

    // Create a new credential issuer
    let issuer = CredentialIssuer::new(vault.clone());

    // Create a new subject attributes
    let attributes = AttributesEntry::single(b"cluster".to_vec(), b"production".to_vec(), None, None).unwrap();

    // Issue a credential
    let credential = issuer.issue_credential(attributes).unwrap();

    // Verify the credential
    let verifier = CredentialVerifier::new(vault.clone());
    let verified = verifier.verify_credential(&credential).unwrap();

    // Print the verification result
    println!("Credential verification result: {:?}", verified);
}
