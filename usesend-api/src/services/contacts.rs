use crate::config::SharedConfig;
use crate::error::ApiResult;
use crate::types::SuccessResponse;
use crate::types::contact::*;

#[derive(Debug, Clone)]
pub struct ContactsSvc(pub(crate) SharedConfig);

impl ContactsSvc {
    pub async fn list(
        &self,
        contact_book_id: &str,
        params: &ListContactsParams,
    ) -> ApiResult<Vec<Contact>> {
        let mut req = self.0.auth(
            self.0.client.get(
                self.0
                    .url(&format!("/v1/contactBooks/{contact_book_id}/contacts")),
            ),
        );
        if let Some(page) = params.page {
            req = req.query(&[("page", page.to_string())]);
        }
        if let Some(limit) = params.limit {
            req = req.query(&[("limit", limit.to_string())]);
        }
        if let Some(emails) = &params.emails {
            req = req.query(&[("emails", emails)]);
        }
        if let Some(ids) = &params.ids {
            req = req.query(&[("ids", ids)]);
        }
        self.0.send_and_parse(req).await
    }

    pub async fn create(
        &self,
        contact_book_id: &str,
        body: &CreateContactRequest,
    ) -> ApiResult<ContactIdResponse> {
        let req = self
            .0
            .auth(
                self.0.client.post(
                    self.0
                        .url(&format!("/v1/contactBooks/{contact_book_id}/contacts")),
                ),
            )
            .json(body);
        self.0.send_and_parse(req).await
    }

    pub async fn get(&self, contact_book_id: &str, contact_id: &str) -> ApiResult<Contact> {
        let req = self.0.auth(self.0.client.get(self.0.url(&format!(
            "/v1/contactBooks/{contact_book_id}/contacts/{contact_id}"
        ))));
        self.0.send_and_parse(req).await
    }

    pub async fn update(
        &self,
        contact_book_id: &str,
        contact_id: &str,
        body: &UpdateContactRequest,
    ) -> ApiResult<ContactIdResponse> {
        let req = self
            .0
            .auth(self.0.client.patch(self.0.url(&format!(
                "/v1/contactBooks/{contact_book_id}/contacts/{contact_id}"
            ))))
            .json(body);
        self.0.send_and_parse(req).await
    }

    pub async fn upsert(
        &self,
        contact_book_id: &str,
        contact_id: &str,
        body: &CreateContactRequest,
    ) -> ApiResult<ContactIdResponse> {
        let req = self
            .0
            .auth(self.0.client.put(self.0.url(&format!(
                "/v1/contactBooks/{contact_book_id}/contacts/{contact_id}"
            ))))
            .json(body);
        self.0.send_and_parse(req).await
    }

    pub async fn delete(
        &self,
        contact_book_id: &str,
        contact_id: &str,
    ) -> ApiResult<SuccessResponse> {
        let req = self.0.auth(self.0.client.delete(self.0.url(&format!(
            "/v1/contactBooks/{contact_book_id}/contacts/{contact_id}"
        ))));
        self.0.send_and_parse(req).await
    }
}
