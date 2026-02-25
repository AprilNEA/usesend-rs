use crate::config::SharedConfig;
use crate::error::ApiResult;
use crate::types::email::*;

#[derive(Debug, Clone)]
pub struct EmailsSvc(pub(crate) SharedConfig);

impl EmailsSvc {
    pub async fn send(&self, body: &SendEmailRequest) -> ApiResult<SendEmailResponse> {
        let req = self
            .0
            .auth(self.0.client.post(self.0.url("/v1/emails")))
            .json(body);
        self.0.send_and_parse(req).await
    }

    pub async fn send_with_idempotency_key(
        &self,
        body: &SendEmailRequest,
        idempotency_key: &str,
    ) -> ApiResult<SendEmailResponse> {
        let req = self
            .0
            .auth(self.0.client.post(self.0.url("/v1/emails")))
            .header("Idempotency-Key", idempotency_key)
            .json(body);
        self.0.send_and_parse(req).await
    }

    pub async fn batch_send(&self, bodies: &[SendEmailRequest]) -> ApiResult<BatchSendResponse> {
        let req = self
            .0
            .auth(self.0.client.post(self.0.url("/v1/emails/batch")))
            .json(bodies);
        self.0.send_and_parse(req).await
    }

    pub async fn batch_send_with_idempotency_key(
        &self,
        bodies: &[SendEmailRequest],
        idempotency_key: &str,
    ) -> ApiResult<BatchSendResponse> {
        let req = self
            .0
            .auth(self.0.client.post(self.0.url("/v1/emails/batch")))
            .header("Idempotency-Key", idempotency_key)
            .json(bodies);
        self.0.send_and_parse(req).await
    }

    pub async fn list(&self, params: &ListEmailsParams) -> ApiResult<ListEmailsResponse> {
        let mut req = self.0.auth(self.0.client.get(self.0.url("/v1/emails")));
        if let Some(page) = &params.page {
            req = req.query(&[("page", page)]);
        }
        if let Some(limit) = &params.limit {
            req = req.query(&[("limit", limit)]);
        }
        if let Some(start_date) = &params.start_date {
            req = req.query(&[("startDate", start_date)]);
        }
        if let Some(end_date) = &params.end_date {
            req = req.query(&[("endDate", end_date)]);
        }
        self.0.send_and_parse(req).await
    }

    pub async fn get(&self, email_id: &str) -> ApiResult<EmailDetail> {
        let req = self.0.auth(
            self.0
                .client
                .get(self.0.url(&format!("/v1/emails/{email_id}"))),
        );
        self.0.send_and_parse(req).await
    }

    pub async fn reschedule(
        &self,
        email_id: &str,
        body: &RescheduleEmailRequest,
    ) -> ApiResult<SendEmailResponse> {
        let req = self
            .0
            .auth(
                self.0
                    .client
                    .patch(self.0.url(&format!("/v1/emails/{email_id}"))),
            )
            .json(body);
        self.0.send_and_parse(req).await
    }

    pub async fn cancel(&self, email_id: &str) -> ApiResult<SendEmailResponse> {
        let req = self.0.auth(
            self.0
                .client
                .post(self.0.url(&format!("/v1/emails/{email_id}/cancel"))),
        );
        self.0.send_and_parse(req).await
    }
}
