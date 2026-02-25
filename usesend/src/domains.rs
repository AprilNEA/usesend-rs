use usesend_api::services::DomainsSvc;
use usesend_api::types::domain::*;
use usesend_api::types::{DeleteResponse, DomainId};
use usesend_api::ApiResult;

/// High-level domains service.
#[derive(Clone)]
pub struct Domains(pub(crate) DomainsSvc);

impl Domains {
    pub async fn list(&self) -> ApiResult<Vec<Domain>> {
        self.0.list().await
    }

    pub async fn create(&self, name: &str, region: &str) -> ApiResult<Domain> {
        self.0
            .create(&CreateDomainRequest {
                name: name.to_string(),
                region: region.to_string(),
            })
            .await
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
