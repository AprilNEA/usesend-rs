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

    pub async fn create(
        &self,
        contact_book_id: &str,
        body: &CreateContactRequest,
    ) -> ApiResult<ContactIdResponse> {
        self.0.create(contact_book_id, body).await
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
