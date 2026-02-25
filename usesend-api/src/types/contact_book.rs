use bon::Builder;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::ContactBookId;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContactBook {
    pub id: ContactBookId,
    pub name: String,
    pub team_id: i64,
    pub properties: HashMap<String, String>,
    pub emoji: String,
    pub created_at: String,
    pub updated_at: String,
    #[serde(rename = "_count", skip_serializing_if = "Option::is_none")]
    pub count: Option<ContactBookCount>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactBookCount {
    pub contacts: i64,
}

// --- Requests ---

#[derive(Debug, Clone, Builder, Serialize, Deserialize)]
pub struct CreateContactBookRequest {
    #[builder(into)]
    pub name: String,
    #[builder(into)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Builder, Serialize, Deserialize)]
pub struct UpdateContactBookRequest {
    #[builder(into)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[builder(into)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<String, String>>,
}
