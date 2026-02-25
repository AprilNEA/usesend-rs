use bon::Builder;
use serde::{Deserialize, Serialize};

use super::DomainId;

// --- Enums ---

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DomainStatus {
    NotStarted,
    Pending,
    Success,
    Failed,
    TemporaryFailure,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum DnsRecordType {
    MX,
    TXT,
}

// --- Types ---

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DnsRecord {
    pub r#type: DnsRecordType,
    pub name: String,
    pub value: String,
    pub ttl: String,
    pub status: DomainStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommended: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Domain {
    pub id: DomainId,
    pub name: String,
    pub team_id: i64,
    pub status: DomainStatus,
    pub public_key: String,
    pub created_at: String,
    pub updated_at: String,
    pub dns_records: Vec<DnsRecord>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub click_tracking: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_tracking: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dkim_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spf_details: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dmarc_added: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_verifying: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subdomain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_checked_time: Option<String>,
}

// --- Request ---

#[derive(Debug, Clone, Builder, Serialize, Deserialize)]
pub struct CreateDomainRequest {
    #[builder(into)]
    pub name: String,
    #[builder(into)]
    pub region: String,
}

// --- Response ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyDomainResponse {
    pub message: String,
}
