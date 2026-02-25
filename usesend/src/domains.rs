use usesend_api::ApiResult;
use usesend_api::services::DomainsSvc;
use usesend_api::types::domain::*;
use usesend_api::types::{DeleteResponse, DomainId};

/// High-level domains service.
#[derive(Clone)]
pub struct Domains(pub(crate) DomainsSvc);

impl Domains {
    pub async fn list(&self) -> ApiResult<Vec<Domain>> {
        self.0.list().await
    }

    pub async fn create(&self, body: &CreateDomainRequest) -> ApiResult<Domain> {
        self.0.create(body).await
    }

    pub async fn get(&self, id: impl Into<DomainId>) -> ApiResult<Domain> {
        self.0.get(id.into()).await
    }

    pub async fn delete(&self, id: impl Into<DomainId>) -> ApiResult<DeleteResponse> {
        self.0.delete(id.into()).await
    }

    pub async fn verify(&self, id: impl Into<DomainId>) -> ApiResult<VerifyDomainResponse> {
        self.0.verify(id.into()).await
    }
}
