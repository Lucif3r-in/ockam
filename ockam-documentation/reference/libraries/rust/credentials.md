# credentials.md

<!-- Import necessary files or entities from the get_started directory -->
import { node, issuer, AttributesEntry } from 'get_started';

<!-- Replace lines 97 to 103 with the updated examples from the get_started directory -->
node.credentials();
issuer.issue_credentials("trust_context".into(), None);

let attributes = AttributesEntry::single(b"cluster".to_vec(), b"production".to_vec(), None, None)?;
