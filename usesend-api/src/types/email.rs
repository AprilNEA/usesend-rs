use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{DomainId, EmailId, StringOrVec};

// --- Enums ---

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EmailEventStatus {
    Scheduled,
    Queued,
    Sent,
    DeliveryDelayed,
    Bounced,
    Rejected,
    RenderingFailure,
    Delivered,
    Opened,
    Clicked,
    Complained,
    Failed,
    Cancelled,
    Suppressed,
}

// --- Types ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attachment {
    pub filename: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmailEvent {
    pub email_id: EmailId,
    pub status: EmailEventStatus,
    pub created_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmailListItem {
    pub id: String,
    pub to: StringOrVec,
    pub from: String,
    pub subject: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub latest_status: Option<EmailEventStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_id: Option<DomainId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<StringOrVec>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cc: Option<StringOrVec>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bcc: Option<StringOrVec>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmailDetail {
    pub id: EmailId,
    pub team_id: i64,
    pub to: StringOrVec,
    pub from: String,
    pub subject: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<StringOrVec>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cc: Option<StringOrVec>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bcc: Option<StringOrVec>,
    pub email_events: Vec<EmailEvent>,
}

// --- Requests ---

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendEmailRequest {
    pub to: StringOrVec,
    pub from: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<StringOrVec>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cc: Option<StringOrVec>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bcc: Option<StringOrVec>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<Attachment>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_reply_to_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RescheduleEmailRequest {
    pub scheduled_at: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListEmailsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_id: Option<StringOrVec>,
}

// --- Responses ---

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendEmailResponse {
    pub email_id: EmailId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchSendResponse {
    pub data: Vec<SendEmailResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListEmailsResponse {
    pub data: Vec<EmailListItem>,
    pub count: i64,
}
