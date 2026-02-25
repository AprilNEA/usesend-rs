use usesend_api::ApiResult;
use usesend_api::services::ContactBooksSvc;
use usesend_api::types::DeleteResponse;
use usesend_api::types::contact_book::*;

/// High-level contact books service.
#[derive(Clone)]
pub struct ContactBooks(pub(crate) ContactBooksSvc);

impl ContactBooks {
    pub async fn list(&self) -> ApiResult<Vec<ContactBook>> {
        self.0.list().await
    }

    pub async fn create(&self, name: &str) -> ApiResult<ContactBook> {
        self.0
            .create(&CreateContactBookRequest {
                name: name.to_string(),
                emoji: None,
                properties: None,
            })
            .await
    }

    pub fn build(&self) -> ContactBookBuilder<'_> {
        ContactBookBuilder::new(&self.0)
    }

    pub async fn get(&self, id: &str) -> ApiResult<ContactBook> {
        self.0.get(id).await
    }

    pub async fn update(
        &self,
        id: &str,
        body: &UpdateContactBookRequest,
    ) -> ApiResult<ContactBook> {
        self.0.update(id, body).await
    }

    pub async fn delete(&self, id: &str) -> ApiResult<DeleteResponse> {
        self.0.delete(id).await
    }
}

/// Builder for creating a contact book.
pub struct ContactBookBuilder<'a> {
    svc: &'a ContactBooksSvc,
    name: Option<String>,
    emoji: Option<String>,
    properties: Option<std::collections::HashMap<String, String>>,
}

impl<'a> ContactBookBuilder<'a> {
    fn new(svc: &'a ContactBooksSvc) -> Self {
        Self {
            svc,
            name: None,
            emoji: None,
            properties: None,
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn emoji(mut self, emoji: impl Into<String>) -> Self {
        self.emoji = Some(emoji.into());
        self
    }

    pub fn property(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.properties
            .get_or_insert_with(std::collections::HashMap::new)
            .insert(key.into(), value.into());
        self
    }

    pub async fn create(self) -> ApiResult<ContactBook> {
        self.svc
            .create(&CreateContactBookRequest {
                name: self.name.expect("`name` is required"),
                emoji: self.emoji,
                properties: self.properties,
            })
            .await
    }
}
