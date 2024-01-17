...
97: use ockam::access_control::AllowAll;
98: use ockam::access_control::IdentityIdAccessControl;
99: use ockam::identity::{AttributesEntry, SecureChannelListenerOptions};
100: use ockam::identity::{CredentialsIssuer, Vault};
101: use ockam::{Context, Result, TcpListenerOptions};
102: use ockam::{Node, TcpTransportExtension};
103: use ockam_api::DefaultAddress;
104: use ockam_vault::{EdDSACurve25519SecretKey, SigningSecret, SoftwareVaultForSigning};
105: 
106: #[ockam::node]
107: async fn main(ctx: Context) -> Result<()> {
108:     let identity_vault = SoftwareVaultForSigning::create().await?;
109:     // Import the signing secret key to the Vault
110:     let secret = identity_vault
111:         .import_key(SigningSecret::EdDSACurve25519(EdDSACurve25519SecretKey::new(
112:             hex::decode("0127359911708ef4de9adaaf27c357501473c4a10a5326a69c1f7f874a0cd82e")
113:                 .unwrap()
114:                 .try_into()
115:                 .unwrap(),
116:         )))
117:         .await?;
118: 
119:     // Create a default Vault but use the signing vault with our secret in it
120:     let mut vault = Vault::create().await?;
121:     vault.identity_vault = identity_vault;
122: 
123:     let node = Node::builder().await?.with_vault(vault).build(&ctx).await?;
124: 
125:     let issuer_identity = hex::decode("81825837830101583285f68200815820afbca9cf5d440147450f9f0d0a038a337b3fe5c17086163f2c54509558b62ef4f41a654cf97d1a7818fc7d8200815840650c4c939b96142546559aed99c52b64aa8a2f7b242b46534f7f8d0c5cc083d2c97210b93e9bca990e9cb9301acc2b634ffb80be314025f9adc870713e6fde0d").unwrap();
126:     let issuer = node.import_private_identity(None, &issuer_identity, &secret).await?;
127:     println!("issuer identifier {}", issuer);
128: 
129:     // Tell the credential issuer about a set of public identifiers that are
130:     // known, in advance, to be members of the production cluster.
131:     let known_identifiers = vec![
132:         "Ie70dc5545d64724880257acb32b8851e7dd1dd57076838991bc343165df71bfe".try_into()?, // Client Identifier
133:         "Ife42b412ecdb7fda4421bd5046e33c1017671ce7a320c3342814f0b99df9ab60".try_into()?, // Server Identifier
134:     ];
135: 
136:     // Tell this credential issuer about the attributes to include in credentials
137:     // that will be issued to each of the above known_identifiers, after and only
138:     // if, they authenticate with their corresponding latest private key.
139:     //
140:     // Since this issuer knows that the above identifiers are for members of the
141:     // production cluster, it will issue a credential that attests to the attribute
142:     // set: [{cluster, production}] for all identifiers in the above list.
143:     //
144:     // For a different application this attested attribute set can be different and
145:     // distinct for each identifier, but for this example we'll keep things simple.
146:     let credential_issuer = CredentialsIssuer::new(
147:         node.identities().identity_attributes_repository(),
148:         node.credentials(),
149:         &issuer,
150:         "trust_context".into(),
151:         None,
152:     );
153: 
154:     let attributes = AttributesEntry::single(b"cluster".to_vec(), b"production".to_vec(), None, None)?;
155:     for identifier in known_identifiers.iter() {
156:         node.identities()
157:             .identity_attributes_repository()
158:             .put_attributes(identifier, attributes.clone())
159:             .await?;
160:     }
161: 
162:     let tcp_listener_options = TcpListenerOptions::new();
163:     let sc_listener_options =
164:         SecureChannelListenerOptions::new().as_consumer(&tcp_listener_options.spawner_flow_control_id());
165:     let sc_listener_flow_control_id = sc_listener_options.spawner_flow_control_id();
166: 
167:     // Start a secure channel listener that only allows channels where the identity
168:     // at the other end of the channel can authenticate with the latest private key
169:     // corresponding to one of the above known public identifiers.
170:     node.create_secure_channel_listener(&issuer, DefaultAddress::SECURE_CHANNEL_LISTENER, sc_listener_options)
171:         .await?;
172: 
173:     // Start a credential issuer worker that will only accept incoming requests from
174:     // authenticated secure channels with our known public identifiers.
175:     let allow_known = IdentityIdAccessControl::new(known_identifiers);
176:     node.flow_controls()
177:         .add_consumer(DefaultAddress::CREDENTIAL_ISSUER, &sc_listener_flow_control_id);
178:     node.start_worker_with_access_control(
179:         DefaultAddress::CREDENTIAL_ISSUER,
180:         credential_issuer,
181:         allow_known,
182:         AllowAll,
183:     )
184:     .await?;
185: 
186:     // Initialize TCP Transport, create a TCP listener, and wait for connections.
187:     let tcp = node.create_tcp_transport().await?;
188:     tcp.listen("127.0.0.1:5000", tcp_listener_options).await?;
189: 
190:     // Don't call node.stop() here so this node runs forever.
191:     println!("issuer started");
192:     Ok(())
193: }
