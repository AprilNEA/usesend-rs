use usesend_api::services::CampaignsSvc;
use usesend_api::types::StringOrVec;
use usesend_api::types::SuccessResponse;
use usesend_api::types::campaign::*;
use usesend_api::ApiResult;

/// High-level campaigns service.
#[derive(Clone)]
pub struct Campaigns(pub(crate) CampaignsSvc);

impl Campaigns {
    pub async fn list(&self, params: &ListCampaignsParams) -> ApiResult<Vec<Campaign>> {
        self.0.list(params).await
    }

    pub fn build(&self) -> CampaignBuilder<'_> {
        CampaignBuilder::new(&self.0)
    }

    pub async fn get(&self, campaign_id: &str) -> ApiResult<Campaign> {
        self.0.get(campaign_id).await
    }

    pub async fn schedule(
        &self,
        campaign_id: &str,
        body: &ScheduleCampaignRequest,
    ) -> ApiResult<SuccessResponse> {
        self.0.schedule(campaign_id, body).await
    }

    pub async fn pause(&self, campaign_id: &str) -> ApiResult<SuccessResponse> {
        self.0.pause(campaign_id).await
    }

    pub async fn resume(&self, campaign_id: &str) -> ApiResult<SuccessResponse> {
        self.0.resume(campaign_id).await
    }
}

/// Builder for creating a campaign.
pub struct CampaignBuilder<'a> {
    svc: &'a CampaignsSvc,
    name: Option<String>,
    from: Option<String>,
    subject: Option<String>,
    contact_book_id: Option<String>,
    content: Option<String>,
    html: Option<String>,
    preview_text: Option<String>,
    reply_to: Option<StringOrVec>,
    cc: Option<StringOrVec>,
    bcc: Option<StringOrVec>,
    send_now: Option<bool>,
    scheduled_at: Option<String>,
    batch_size: Option<i64>,
}

impl<'a> CampaignBuilder<'a> {
    fn new(svc: &'a CampaignsSvc) -> Self {
        Self {
            svc,
            name: None,
            from: None,
            subject: None,
            contact_book_id: None,
            content: None,
            html: None,
            preview_text: None,
            reply_to: None,
            cc: None,
            bcc: None,
            send_now: None,
            scheduled_at: None,
            batch_size: None,
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
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

    pub fn contact_book_id(mut self, id: impl Into<String>) -> Self {
        self.contact_book_id = Some(id.into());
        self
    }

    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.content = Some(content.into());
        self
    }

    pub fn html(mut self, html: impl Into<String>) -> Self {
        self.html = Some(html.into());
        self
    }

    pub fn preview_text(mut self, text: impl Into<String>) -> Self {
        self.preview_text = Some(text.into());
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

    pub fn send_now(mut self, send_now: bool) -> Self {
        self.send_now = Some(send_now);
        self
    }

    pub fn scheduled_at(mut self, at: impl Into<String>) -> Self {
        self.scheduled_at = Some(at.into());
        self
    }

    pub fn batch_size(mut self, size: i64) -> Self {
        self.batch_size = Some(size);
        self
    }

    pub async fn create(self) -> ApiResult<Campaign> {
        self.svc
            .create(&CreateCampaignRequest {
                name: self.name.expect("`name` is required"),
                from: self.from.expect("`from` is required"),
                subject: self.subject.expect("`subject` is required"),
                contact_book_id: self.contact_book_id.expect("`contact_book_id` is required"),
                content: self.content,
                html: self.html,
                preview_text: self.preview_text,
                reply_to: self.reply_to,
                cc: self.cc,
                bcc: self.bcc,
                send_now: self.send_now,
                scheduled_at: self.scheduled_at,
                batch_size: self.batch_size,
            })
            .await
    }
}
