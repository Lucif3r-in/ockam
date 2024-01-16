# credentials.md

...

97: 
98: let issuer_identity = hex::decode("81825837830101583285f68200815820afbca9cf5d440147450f9f0d0a038a337b3fe5c17086163f2c54509558b62ef4f41a654cf97d1a7818fc7d8200815840650c4c939b96142546559aed99c52b64aa8a2f7b242b46534f7f8d0c5cc083d2c97210b93e9bca990e9cb9301acc2b634ffb80be314025f9adc870713e6fde0d").unwrap();
99: let issuer = node.import_private_identity(None, &issuer_identity, &secret).await?;
100: println!("issuer identifier {}", issuer);
101: 
102: // Tell the credential issuer about a set of public identifiers that are
103: // known, in advance, to be members of the production cluster.
104: let known_identifiers = vec![
105:     "Ie70dc5545d64724880257acb32b8851e7dd1dd57076838991bc343165df71bfe".try_into()?, // Client Identifier
106:     "Ife42b412ecdb7fda4421bd5046e33c1017671ce7a320c3342814f0b99df9ab60".try_into()?, // Server Identifier
107: ];
108: 
109: // Tell this credential issuer about the attributes to include in credentials
110: // that will be issued to each of the above known_identifiers, after and only
111: // if, they authenticate with their corresponding latest private key.
112: //
113: // Since this issuer knows that the above identifiers are for members of the
114: // production cluster, it will issue a credential that attests to the attribute
115: // set: [{cluster, production}] for all identifiers in the above list.
116: //
117: // For a different application this attested attribute set can be different and
118: // distinct for each identifier, but for this example we'll keep things simple.
119: let credential_issuer = CredentialsIssuer::new(
120:     node.identities().identity_attributes_repository(),
121:     node.credentials(),
122:     &issuer,
123:     "trust_context".into(),
124:     None,
125: );
126: 
127: let attributes = AttributesEntry::single(b"cluster".to_vec(), b"production".to_vec(), None, None)?;
128: for identifier in known_identifiers.iter() {
129:     node.identities()
130:         .identity_attributes_repository()
131:         .put_attributes(identifier, attributes.clone())
132:         .await?;
133: }
134: 
135: let tcp_listener_options = TcpListenerOptions::new();
136: let sc_listener_options =
137:     SecureChannelListenerOptions::new().as_consumer(&tcp_listener_options.spawner_flow_control_id());
138: let sc_listener_flow_control_id = sc_listener_options.spawner_flow_control_id();
139: 
140: // Start a secure channel listener that only allows channels where the identity
141: // at the other end of the channel can authenticate with the latest private key
142: // corresponding to one of the above known public identifiers.
143: node.create_secure_channel_listener(&issuer, DefaultAddress::SECURE_CHANNEL_LISTENER, sc_listener_options)
144:     .await?;
145: 
146: // Start a credential issuer worker that will only accept incoming requests from
147: // authenticated secure channels with our known public identifiers.
148: let allow_known = IdentityIdAccessControl::new(known_identifiers);
149: node.flow_controls()
150:     .add_consumer(DefaultAddress::CREDENTIAL_ISSUER, &sc_listener_flow_control_id);
151: node.start_worker_with_access_control(
152:     DefaultAddress::CREDENTIAL_ISSUER,
153:     credential_issuer,
154:     allow_known,
155:     AllowAll,
156: )
157: .await?;
158: 
159: // Initialize TCP Transport, create a TCP listener, and wait for connections.
160: let tcp = node.create_tcp_transport().await?;
161: tcp.listen("127.0.0.1:5000", tcp_listener_options).await?;
162: 
163: // Don't call node.stop() here so this node runs forever.
164: println!("issuer started");

...
