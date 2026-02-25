use usesend_api::ApiResult;
use usesend_api::services::EmailsSvc;
use usesend_api::types::email::*;

/// High-level emails service.
#[derive(Clone)]
pub struct Emails(pub(crate) EmailsSvc);

impl Emails {
    pub async fn send(&self, body: &SendEmailRequest) -> ApiResult<SendEmailResponse> {
        self.0.send(body).await
    }

    pub async fn send_with_idempotency_key(
        &self,
        body: &SendEmailRequest,
        idempotency_key: &str,
    ) -> ApiResult<SendEmailResponse> {
        self.0
            .send_with_idempotency_key(body, idempotency_key)
            .await
    }

    pub async fn batch_send(&self, bodies: &[SendEmailRequest]) -> ApiResult<BatchSendResponse> {
        self.0.batch_send(bodies).await
    }

    pub async fn list(&self, params: &ListEmailsParams) -> ApiResult<ListEmailsResponse> {
        self.0.list(params).await
    }

    pub async fn get(&self, email_id: &str) -> ApiResult<EmailDetail> {
        self.0.get(email_id).await
    }

    pub async fn reschedule(
        &self,
        email_id: &str,
        scheduled_at: &str,
    ) -> ApiResult<SendEmailResponse> {
        self.0
            .reschedule(
                email_id,
                &RescheduleEmailRequest {
                    scheduled_at: scheduled_at.to_string(),
                },
            )
            .await
    }

    pub async fn cancel(&self, email_id: &str) -> ApiResult<SendEmailResponse> {
        self.0.cancel(email_id).await
    }
}
