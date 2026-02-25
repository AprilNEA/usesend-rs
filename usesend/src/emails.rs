use std::collections::HashMap;
use usesend_api::ApiResult;
use usesend_api::services::EmailsSvc;
use usesend_api::types::StringOrVec;
use usesend_api::types::email::*;

/// High-level emails service.
#[derive(Clone)]
pub struct Emails(pub(crate) EmailsSvc);

impl Emails {
    /// Start building an email to send.
    ///
    /// ```no_run
    /// # async fn example() -> usesend_api::ApiResult<()> {
    /// let client = usesend::UseSend::new("api_key");
    /// let result = client.emails.build()
    ///     .from("hello@example.com")
    ///     .to("user@example.com")
    ///     .subject("Hello!")
    ///     .html("<h1>Hi</h1>")
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn build(&self) -> EmailBuilder<'_> {
        EmailBuilder::new(&self.0)
    }

    pub async fn send(&self, body: &SendEmailRequest) -> ApiResult<SendEmailResponse> {
        self.0.send(body).await
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

/// Ergonomic builder for sending emails.
pub struct EmailBuilder<'a> {
    svc: &'a EmailsSvc,
    to: Option<StringOrVec>,
    from: Option<String>,
    subject: Option<String>,
    template_id: Option<String>,
    variables: Option<HashMap<String, String>>,
    reply_to: Option<StringOrVec>,
    cc: Option<StringOrVec>,
    bcc: Option<StringOrVec>,
    text: Option<String>,
    html: Option<String>,
    headers: Option<HashMap<String, String>>,
    attachments: Vec<Attachment>,
    scheduled_at: Option<String>,
    in_reply_to_id: Option<String>,
    idempotency_key: Option<String>,
}

impl<'a> EmailBuilder<'a> {
    pub(crate) fn new(svc: &'a EmailsSvc) -> Self {
        Self {
            svc,
            to: None,
            from: None,
            subject: None,
            template_id: None,
            variables: None,
            reply_to: None,
            cc: None,
            bcc: None,
            text: None,
            html: None,
            headers: None,
            attachments: Vec::new(),
            scheduled_at: None,
            in_reply_to_id: None,
            idempotency_key: None,
        }
    }

    pub fn to(mut self, to: impl Into<StringOrVec>) -> Self {
        self.to = Some(to.into());
        self
    }

    pub fn from(mut self, from: impl Into<String>) -> Self {
        self.from = Some(from.into());
        self
    }

    pub fn subject(mut self, subject: impl Into<String>) -> Self {
        self.subject = Some(subject.into());
        self
    }

    pub fn template_id(mut self, template_id: impl Into<String>) -> Self {
        self.template_id = Some(template_id.into());
        self
    }

    pub fn variable(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.variables
            .get_or_insert_with(HashMap::new)
            .insert(key.into(), value.into());
        self
    }

    pub fn variables(mut self, vars: HashMap<String, String>) -> Self {
        self.variables = Some(vars);
        self
    }

    pub fn reply_to(mut self, reply_to: impl Into<StringOrVec>) -> Self {
        self.reply_to = Some(reply_to.into());
        self
    }

    pub fn cc(mut self, cc: impl Into<StringOrVec>) -> Self {
        self.cc = Some(cc.into());
        self
    }

    pub fn bcc(mut self, bcc: impl Into<StringOrVec>) -> Self {
        self.bcc = Some(bcc.into());
        self
    }

    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn html(mut self, html: impl Into<String>) -> Self {
        self.html = Some(html.into());
        self
    }

    pub fn header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers
            .get_or_insert_with(HashMap::new)
            .insert(key.into(), value.into());
        self
    }

    pub fn attachment(mut self, filename: impl Into<String>, content: impl Into<String>) -> Self {
        self.attachments.push(Attachment {
            filename: filename.into(),
            content: content.into(),
        });
        self
    }

    pub fn scheduled_at(mut self, scheduled_at: impl Into<String>) -> Self {
        self.scheduled_at = Some(scheduled_at.into());
        self
    }

    pub fn in_reply_to(mut self, id: impl Into<String>) -> Self {
        self.in_reply_to_id = Some(id.into());
        self
    }

    pub fn idempotency_key(mut self, key: impl Into<String>) -> Self {
        self.idempotency_key = Some(key.into());
        self
    }

    /// Send the email.
    pub async fn send(self) -> ApiResult<SendEmailResponse> {
        let req = SendEmailRequest {
            to: self.to.expect("`to` is required"),
            from: self.from.expect("`from` is required"),
            subject: self.subject,
            template_id: self.template_id,
            variables: self.variables,
            reply_to: self.reply_to,
            cc: self.cc,
            bcc: self.bcc,
            text: self.text,
            html: self.html,
            headers: self.headers,
            attachments: if self.attachments.is_empty() {
                None
            } else {
                Some(self.attachments)
            },
            scheduled_at: self.scheduled_at,
            in_reply_to_id: self.in_reply_to_id,
        };

        match self.idempotency_key {
            Some(key) => self.svc.send_with_idempotency_key(&req, &key).await,
            None => self.svc.send(&req).await,
        }
    }
}
