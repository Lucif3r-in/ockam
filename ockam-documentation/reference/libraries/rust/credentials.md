## ockam-documentation/reference/libraries/rust/credentials.md

...

97: let credential_issuer = CredentialsIssuer::new(
98:     node.identities().identity_attributes_repository(),
99:     node.credentials(),
100:    &issuer,
101:    "trust_context".into(),
102:    None,
103: );

...
