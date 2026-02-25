use serde::{Deserialize, Serialize};

use super::{CampaignId, ContactBookId, StringOrVec};

// --- Enums ---

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CampaignStatus {
    Draft,
    Scheduled,
    InProgress,
    Paused,
    Completed,
    Cancelled,
}

// --- Types ---

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Campaign {
    pub id: CampaignId,
    pub name: String,
    pub from: String,
    pub subject: String,
    pub status: CampaignStatus,
    pub created_at: String,
    pub updated_at: String,
    pub scheduled_at: Option<String>,
    pub preview_text: Option<String>,
    pub contact_book_id: Option<ContactBookId>,
    pub html: Option<String>,
    pub content: Option<String>,
    pub batch_size: i64,
    pub batch_window_minutes: i64,
    pub total: i64,
    pub sent: i64,
    pub delivered: i64,
    pub opened: i64,
    pub clicked: i64,
    pub unsubscribed: i64,
    pub bounced: i64,
    pub hard_bounced: i64,
    pub complained: i64,
    pub reply_to: Vec<String>,
    pub cc: Vec<String>,
    pub bcc: Vec<String>,
}

// --- Requests ---

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCampaignRequest {
    pub name: String,
    pub from: String,
    pub subject: String,
    pub contact_book_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<StringOrVec>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cc: Option<StringOrVec>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bcc: Option<StringOrVec>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_now: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScheduleCampaignRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<i64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListCampaignsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<CampaignStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
}
