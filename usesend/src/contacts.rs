use usesend_api::ApiResult;
use usesend_api::services::ContactsSvc;
use usesend_api::types::SuccessResponse;
use usesend_api::types::contact::*;

/// High-level contacts service.
#[derive(Clone)]
pub struct Contacts(pub(crate) ContactsSvc);

impl Contacts {
    pub async fn list(
        &self,
        contact_book_id: &str,
        params: &ListContactsParams,
    ) -> ApiResult<Vec<Contact>> {
        self.0.list(contact_book_id, params).await
    }

    pub fn build(&self, contact_book_id: impl Into<String>) -> ContactBuilder<'_> {
        ContactBuilder::new(&self.0, contact_book_id.into())
    }

    pub async fn get(&self, contact_book_id: &str, contact_id: &str) -> ApiResult<Contact> {
        self.0.get(contact_book_id, contact_id).await
    }

    pub async fn update(
        &self,
        contact_book_id: &str,
        contact_id: &str,
        body: &UpdateContactRequest,
    ) -> ApiResult<ContactIdResponse> {
        self.0.update(contact_book_id, contact_id, body).await
    }

    pub async fn upsert(
        &self,
        contact_book_id: &str,
        contact_id: &str,
        body: &CreateContactRequest,
    ) -> ApiResult<ContactIdResponse> {
        self.0.upsert(contact_book_id, contact_id, body).await
    }

    pub async fn delete(
        &self,
        contact_book_id: &str,
        contact_id: &str,
    ) -> ApiResult<SuccessResponse> {
        self.0.delete(contact_book_id, contact_id).await
    }
}

/// Builder for creating a contact.
pub struct ContactBuilder<'a> {
    svc: &'a ContactsSvc,
    contact_book_id: String,
    email: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    properties: Option<std::collections::HashMap<String, String>>,
    subscribed: Option<bool>,
}

impl<'a> ContactBuilder<'a> {
    fn new(svc: &'a ContactsSvc, contact_book_id: String) -> Self {
        Self {
            svc,
            contact_book_id,
            email: None,
            first_name: None,
            last_name: None,
            properties: None,
            subscribed: None,
        }
    }

    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.email = Some(email.into());
        self
    }

    pub fn first_name(mut self, name: impl Into<String>) -> Self {
        self.first_name = Some(name.into());
        self
    }

    pub fn last_name(mut self, name: impl Into<String>) -> Self {
        self.last_name = Some(name.into());
        self
    }

    pub fn property(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.properties
            .get_or_insert_with(std::collections::HashMap::new)
            .insert(key.into(), value.into());
        self
    }

    pub fn subscribed(mut self, subscribed: bool) -> Self {
        self.subscribed = Some(subscribed);
        self
    }

    pub async fn create(self) -> ApiResult<ContactIdResponse> {
        self.svc
            .create(
                &self.contact_book_id,
                &CreateContactRequest {
                    email: self.email.expect("`email` is required"),
                    first_name: self.first_name,
                    last_name: self.last_name,
                    properties: self.properties,
                    subscribed: self.subscribed,
                },
            )
            .await
    }
}
