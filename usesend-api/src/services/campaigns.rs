use crate::config::SharedConfig;
use crate::error::ApiResult;
use crate::types::SuccessResponse;
use crate::types::campaign::*;

#[derive(Debug, Clone)]
pub struct CampaignsSvc(pub(crate) SharedConfig);

impl CampaignsSvc {
    pub async fn list(&self, params: &ListCampaignsParams) -> ApiResult<Vec<Campaign>> {
        let mut req = self.0.auth(self.0.client.get(self.0.url("/v1/campaigns")));
        if let Some(page) = &params.page {
            req = req.query(&[("page", page)]);
        }
        if let Some(search) = &params.search {
            req = req.query(&[("search", search)]);
        }
        self.0.send_and_parse(req).await
    }

    pub async fn create(&self, body: &CreateCampaignRequest) -> ApiResult<Campaign> {
        let req = self
            .0
            .auth(self.0.client.post(self.0.url("/v1/campaigns")))
            .json(body);
        self.0.send_and_parse(req).await
    }

    pub async fn get(&self, campaign_id: &str) -> ApiResult<Campaign> {
        let req = self
            .0
            .auth(self.0.client.get(self.0.url(&format!("/v1/campaigns/{campaign_id}"))));
        self.0.send_and_parse(req).await
    }

    pub async fn schedule(
        &self,
        campaign_id: &str,
        body: &ScheduleCampaignRequest,
    ) -> ApiResult<SuccessResponse> {
        let req = self
            .0
            .auth(
                self.0
                    .client
                    .post(self.0.url(&format!("/v1/campaigns/{campaign_id}/schedule"))),
            )
            .json(body);
        self.0.send_and_parse(req).await
    }

    pub async fn pause(&self, campaign_id: &str) -> ApiResult<SuccessResponse> {
        let req = self.0.auth(
            self.0
                .client
                .post(self.0.url(&format!("/v1/campaigns/{campaign_id}/pause"))),
        );
        self.0.send_and_parse(req).await
    }

    pub async fn resume(&self, campaign_id: &str) -> ApiResult<SuccessResponse> {
        let req = self.0.auth(
            self.0
                .client
                .post(self.0.url(&format!("/v1/campaigns/{campaign_id}/resume"))),
        );
        self.0.send_and_parse(req).await
    }
}
