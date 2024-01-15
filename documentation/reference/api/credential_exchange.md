## Credential exchange API

Allows client nodes to present credentials (one-way exchange)

Worker address: "credentials"

Implemented in `Ockam.Services.API.CredentialExchange`


#### Present credential
Method: POST \Path: "actions/present" \Request: Credential \Response: ""

Errors:
- 400 - credential is invalid
- 400 - secure channel required

let authority_node = NodeManager::authority_node_client(\n&tcp,\nnode.secure_channels().clone(),\n&issuer,

// Updated examples from get_started.rs
let authority_node = NodeManager::authority_node(
    &tcp,
    node.secure_channels().clone(),
    &issuer,
    // Add any additional parameters here
);
