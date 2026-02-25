use bon::Builder;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{ContactBookId, ContactId};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contact {
    pub id: ContactId,
    pub email: String,
    pub contact_book_id: ContactBookId,
    pub properties: HashMap<String, String>,
    pub created_at: String,
    pub updated_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribed: Option<bool>,
}

// --- Requests ---

#[derive(Debug, Clone, Builder, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateContactRequest {
    #[builder(into)]
    pub email: String,
    #[builder(into)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[builder(into)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribed: Option<bool>,
}

#[derive(Debug, Clone, Builder, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateContactRequest {
    #[builder(into)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[builder(into)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribed: Option<bool>,
}

#[derive(Debug, Clone, Default, Builder, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListContactsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[builder(into)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emails: Option<String>,
    #[builder(into)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<String>,
}

// --- Responses ---

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContactIdResponse {
    pub contact_id: ContactId,
}
