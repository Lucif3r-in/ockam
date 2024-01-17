// Updated code block from ockam/examples/rust/get_started/credentials.rs
node.credentials(),
&issuer,
"trust_context".into(),
None,
);

let attributes = AttributesEntry::single(b"cluster".to_vec(), b"production".to_vec(), None, None)?;
