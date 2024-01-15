// The credential issuer already knows the public identifier of this identity
// as a member of the production cluster so it returns a signed credential
// attesting to that knowledge.
let authority_node = NodeManager::authority_node(
    &tcp,
    node.secure_channels().clone(),
    &issuer,
    // ...
);

// The authority node already knows the public identifier of the client
// as a member of the production cluster so it returns a signed credential
// attesting to that knowledge.
let authority_node = NodeManager::authority_node(
    &tcp,
    node.secure_channels().clone(),
    &issuer,
    // ...
);
