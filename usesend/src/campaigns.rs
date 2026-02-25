use usesend_api::ApiResult;
use usesend_api::services::CampaignsSvc;
use usesend_api::types::SuccessResponse;
use usesend_api::types::campaign::*;

/// High-level campaigns service.
#[derive(Clone)]
pub struct Campaigns(pub(crate) CampaignsSvc);

impl Campaigns {
    pub async fn list(&self, params: &ListCampaignsParams) -> ApiResult<Vec<Campaign>> {
        self.0.list(params).await
    }

    pub async fn create(&self, body: &CreateCampaignRequest) -> ApiResult<Campaign> {
        self.0.create(body).await
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
