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

    pub async fn create(&self, body: &CreateContactBookRequest) -> ApiResult<ContactBook> {
        self.0.create(body).await
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
