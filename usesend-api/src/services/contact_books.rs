use crate::config::SharedConfig;
use crate::error::ApiResult;
use crate::types::DeleteResponse;
use crate::types::contact_book::*;

#[derive(Debug, Clone)]
pub struct ContactBooksSvc(pub(crate) SharedConfig);

impl ContactBooksSvc {
    pub async fn list(&self) -> ApiResult<Vec<ContactBook>> {
        let req = self
            .0
            .auth(self.0.client.get(self.0.url("/v1/contactBooks")));
        self.0.send_and_parse(req).await
    }

    pub async fn create(&self, body: &CreateContactBookRequest) -> ApiResult<ContactBook> {
        let req = self
            .0
            .auth(self.0.client.post(self.0.url("/v1/contactBooks")))
            .json(body);
        self.0.send_and_parse(req).await
    }

    pub async fn get(&self, id: &str) -> ApiResult<ContactBook> {
        let req = self.0.auth(
            self.0
                .client
                .get(self.0.url(&format!("/v1/contactBooks/{id}"))),
        );
        self.0.send_and_parse(req).await
    }

    pub async fn update(
        &self,
        id: &str,
        body: &UpdateContactBookRequest,
    ) -> ApiResult<ContactBook> {
        let req = self
            .0
            .auth(
                self.0
                    .client
                    .patch(self.0.url(&format!("/v1/contactBooks/{id}"))),
            )
            .json(body);
        self.0.send_and_parse(req).await
    }

    pub async fn delete(&self, id: &str) -> ApiResult<DeleteResponse> {
        let req = self.0.auth(
            self.0
                .client
                .delete(self.0.url(&format!("/v1/contactBooks/{id}"))),
        );
        self.0.send_and_parse(req).await
    }
}
