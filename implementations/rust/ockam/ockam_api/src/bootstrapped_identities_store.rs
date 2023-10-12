use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use serde_json as json;
use tracing::trace;

use ockam::identity::models::ChangeHistory;
use ockam::identity::utils::now;
use ockam::identity::{
    AttributesEntry, Identifier, IdentitiesReader, IdentitiesRepository, IdentitiesWriter,
    Identity, IdentityAttributesReader, IdentityAttributesWriter, NamedIdentity,
};
use ockam_core::async_trait;
use ockam_core::compat::sync::Arc;
use ockam_core::compat::{collections::HashMap, string::String, vec::Vec};
use ockam_core::errcode::{Kind, Origin};
use ockam_core::Result;

#[derive(Clone)]
pub struct BootstrapedIdentityStore {
    bootstrapped: Arc<dyn IdentityAttributesReader>,
    repository: Arc<dyn IdentitiesRepository>,
}

impl BootstrapedIdentityStore {
    pub fn new(
        bootstrapped: Arc<dyn IdentityAttributesReader>,
        repository: Arc<dyn IdentitiesRepository>,
    ) -> Self {
        Self {
            bootstrapped,
            repository,
        }
    }
}

#[async_trait]
impl IdentityAttributesReader for BootstrapedIdentityStore {
    async fn get_attributes(&self, identity_id: &Identifier) -> Result<Option<AttributesEntry>> {
        trace! {
            target: "ockam_api::bootstrapped_identities_store",
            id     = %identity_id,
            "get_attributes"
        }
        match self.bootstrapped.get_attributes(identity_id).await? {
            None => self.repository.get_attributes(identity_id).await,
            Some(x) => Ok(Some(x)),
        }
    }

    async fn list(&self) -> Result<Vec<(Identifier, AttributesEntry)>> {
        let mut l = self.repository.list().await?;
        let mut l2 = self.bootstrapped.list().await?;
        l.append(&mut l2);
        Ok(l)
    }
}

#[async_trait]
impl IdentityAttributesWriter for BootstrapedIdentityStore {
    async fn put_attributes(&self, sender: &Identifier, entry: AttributesEntry) -> Result<()> {
        trace! {
            target: "ockam_api::bootstrapped_identities_store",
            id     = %sender,
            "put_attributes"
        }
        match self.bootstrapped.get_attributes(sender).await? {
            None => self.repository.put_attributes(sender, entry).await,
            // FIXME: allow overwriting the attributes?
            Some(_) => Err(ockam_core::Error::new(
                Origin::Identity,
                Kind::AlreadyExists,
                "cant write attributes for a bootstrapped identity",
            )),
        }
    }

    async fn put_attribute_value(
        &self,
        subject: &Identifier,
        attribute_name: Vec<u8>,
        attribute_value: Vec<u8>,
    ) -> Result<()> {
        self.repository
            .put_attribute_value(subject, attribute_name, attribute_value)
            .await
    }

    async fn delete(&self, identity: &Identifier) -> Result<()> {
        self.repository.delete(identity).await
    }
}

#[async_trait]
impl IdentitiesReader for BootstrapedIdentityStore {
    async fn get_change_history_optional(
        &self,
        identifier: &Identifier,
    ) -> Result<Option<ChangeHistory>> {
        self.repository
            .get_change_history_optional(identifier)
            .await
    }

    async fn get_change_history(&self, identifier: &Identifier) -> Result<ChangeHistory> {
        self.repository.get_change_history(identifier).await
    }

    async fn get_identifier_by_name(&self, name: &str) -> Result<Option<Identifier>> {
        self.repository.get_identifier_by_name(name).await
    }

    async fn get_default_identifier(&self) -> Result<Option<Identifier>> {
        self.repository.get_default_identifier().await
    }

    async fn get_named_identities(&self) -> Result<Vec<NamedIdentity>> {
        self.repository.get_named_identities().await
    }

    async fn get_named_identity(&self, name: &str) -> Result<Option<NamedIdentity>> {
        self.repository.get_named_identity(name).await
    }

    async fn get_default_named_identity(&self) -> Result<Option<NamedIdentity>> {
        self.repository.get_default_named_identity().await
    }

    async fn get_default_identity_name(&self) -> Result<Option<String>> {
        self.repository.get_default_identity_name().await
    }

    async fn is_default_identity_by_name(&self, name: &str) -> Result<bool> {
        self.repository.is_default_identity_by_name(name).await
    }
}

#[async_trait]
impl IdentitiesWriter for BootstrapedIdentityStore {
    async fn store_identity(&self, identity: &Identity) -> Result<()> {
        self.repository.store_identity(identity).await
    }

    async fn name_identity(&self, identifier: &Identifier, name: &str) -> Result<()> {
        self.repository.name_identity(identifier, name).await
    }

    async fn set_as_default(&self, identifier: &Identifier) -> Result<()> {
        self.repository.set_as_default(identifier).await
    }

    async fn set_as_default_by_name(&self, name: &str) -> Result<()> {
        self.repository.set_as_default_by_name(name).await
    }

    async fn update_identity(&self, identity: &Identity) -> Result<()> {
        self.repository.update_identity(identity).await
    }

    async fn delete_identity(&self, identifier: &Identifier) -> Result<()> {
        self.repository.delete_identity(identifier).await
    }

    async fn delete_identity_by_name(&self, name: &str) -> Result<()> {
        self.repository.delete_identity_by_name(name).await
    }
}

impl IdentitiesRepository for BootstrapedIdentityStore {
    fn as_attributes_reader(&self) -> Arc<dyn IdentityAttributesReader> {
        Arc::new(self.clone())
    }

    fn as_attributes_writer(&self) -> Arc<dyn IdentityAttributesWriter> {
        Arc::new(self.clone())
    }

    fn as_identities_reader(&self) -> Arc<dyn IdentitiesReader> {
        Arc::new(self.clone())
    }

    fn as_identities_writer(&self) -> Arc<dyn IdentitiesWriter> {
        Arc::new(self.clone())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PreTrustedIdentities {
    Fixed(HashMap<Identifier, AttributesEntry>),
    ReloadFrom(PathBuf),
}

impl PreTrustedIdentities {
    pub fn new_from_disk(path: PathBuf, reload: bool) -> Result<Self> {
        if reload {
            Ok(PreTrustedIdentities::ReloadFrom(path))
        } else {
            Ok(PreTrustedIdentities::Fixed(Self::parse_from_disk(&path)?))
        }
    }

    pub fn new_from_string(entries: &str) -> Result<Self> {
        Ok(Self::new_from_hashmap(Self::parse(entries)?))
    }

    pub fn new_from_hashmap(entries: HashMap<Identifier, AttributesEntry>) -> Self {
        PreTrustedIdentities::Fixed(entries)
    }

    fn parse_from_disk(path: &PathBuf) -> Result<HashMap<Identifier, AttributesEntry>> {
        let contents = std::fs::read_to_string(path)
            .map_err(|e| ockam_core::Error::new(Origin::Other, Kind::Io, e))?;
        Self::parse(&contents)
    }

    fn parse(entries: &str) -> Result<HashMap<Identifier, AttributesEntry>> {
        let raw_map = json::from_str::<HashMap<Identifier, HashMap<String, String>>>(entries)
            .map_err(|e| ockam_core::Error::new(Origin::Other, Kind::Invalid, e))?;
        let now = now()?;
        Ok(raw_map
            .into_iter()
            .map(|(identity_id, raw_attrs)| {
                let attrs = raw_attrs
                    .into_iter()
                    .map(|(k, v)| (k.as_bytes().to_vec(), v.as_bytes().to_vec()))
                    .collect();
                (identity_id, AttributesEntry::new(attrs, now, None, None))
            })
            .collect())
    }
}

impl From<HashMap<Identifier, AttributesEntry>> for PreTrustedIdentities {
    fn from(h: HashMap<Identifier, AttributesEntry>) -> PreTrustedIdentities {
        PreTrustedIdentities::Fixed(h)
    }
}

#[async_trait]
impl IdentityAttributesReader for PreTrustedIdentities {
    async fn get_attributes(&self, identity_id: &Identifier) -> Result<Option<AttributesEntry>> {
        match self {
            PreTrustedIdentities::Fixed(trusted) => Ok(trusted.get(identity_id).cloned()),
            PreTrustedIdentities::ReloadFrom(path) => {
                Ok(Self::parse_from_disk(path)?.get(identity_id).cloned())
            }
        }
    }

    async fn list(&self) -> Result<Vec<(Identifier, AttributesEntry)>> {
        match self {
            PreTrustedIdentities::Fixed(trusted) => Ok(trusted
                .into_iter()
                .map(|(k, v)| (k.to_owned(), v.to_owned()))
                .collect()),
            PreTrustedIdentities::ReloadFrom(path) => Ok(Self::parse_from_disk(path)?
                .into_iter()
                .map(|(k, v)| (k, v))
                .collect()),
        }
    }
}
