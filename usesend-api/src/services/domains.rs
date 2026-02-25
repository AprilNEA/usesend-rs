use crate::config::SharedConfig;
use crate::error::ApiResult;
use crate::types::domain::*;
use crate::types::{DomainId, DeleteResponse};

#[derive(Debug, Clone)]
pub struct DomainsSvc(pub(crate) SharedConfig);

impl DomainsSvc {
    pub async fn list(&self) -> ApiResult<Vec<Domain>> {
        let req = self.0.auth(self.0.client.get(self.0.url("/v1/domains")));
        self.0.send_and_parse(req).await
    }

    pub async fn create(&self, body: &CreateDomainRequest) -> ApiResult<Domain> {
        let req = self
            .0
            .auth(self.0.client.post(self.0.url("/v1/domains")))
            .json(body);
        self.0.send_and_parse(req).await
    }

    pub async fn get(&self, id: DomainId) -> ApiResult<Domain> {
        let req = self
            .0
            .auth(self.0.client.get(self.0.url(&format!("/v1/domains/{id}"))));
        self.0.send_and_parse(req).await
    }

    pub async fn delete(&self, id: DomainId) -> ApiResult<DeleteResponse> {
        let req = self
            .0
            .auth(self.0.client.delete(self.0.url(&format!("/v1/domains/{id}"))));
        self.0.send_and_parse(req).await
    }

    pub async fn verify(&self, id: DomainId) -> ApiResult<VerifyDomainResponse> {
        let req = self
            .0
            .auth(self.0.client.put(self.0.url(&format!("/v1/domains/{id}/verify"))));
        self.0.send_and_parse(req).await
    }
}
