use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub mod campaign;
pub mod contact;
pub mod contact_book;
pub mod domain;
pub mod email;

macro_rules! define_id_type {
    ($name:ident, String) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(pub String);

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                &self.0
            }
        }

        impl std::ops::Deref for $name {
            type Target = str;
            fn deref(&self) -> &str {
                &self.0
            }
        }

        impl From<String> for $name {
            fn from(s: String) -> Self {
                Self(s)
            }
        }

        impl From<&str> for $name {
            fn from(s: &str) -> Self {
                Self(s.to_string())
            }
        }
    };
    ($name:ident, i64) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(pub i64);

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }

        impl From<i64> for $name {
            fn from(id: i64) -> Self {
                Self(id)
            }
        }
    };
}

define_id_type!(DomainId, i64);
define_id_type!(EmailId, String);
define_id_type!(ContactBookId, String);
define_id_type!(ContactId, String);
define_id_type!(CampaignId, String);

/// A value that can be either a single string or a list of strings.
/// Used for `to`, `cc`, `bcc`, `replyTo` fields.
#[derive(Debug, Clone, PartialEq)]
pub enum StringOrVec {
    Single(String),
    Multiple(Vec<String>),
}

impl From<&str> for StringOrVec {
    fn from(s: &str) -> Self {
        StringOrVec::Single(s.to_string())
    }
}

impl From<String> for StringOrVec {
    fn from(s: String) -> Self {
        StringOrVec::Single(s)
    }
}

impl From<Vec<String>> for StringOrVec {
    fn from(v: Vec<String>) -> Self {
        StringOrVec::Multiple(v)
    }
}

impl Serialize for StringOrVec {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            StringOrVec::Single(s) => serializer.serialize_str(s),
            StringOrVec::Multiple(v) => v.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for StringOrVec {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum Helper {
            Single(String),
            Multiple(Vec<String>),
        }
        match Helper::deserialize(deserializer)? {
            Helper::Single(s) => Ok(StringOrVec::Single(s)),
            Helper::Multiple(v) => Ok(StringOrVec::Multiple(v)),
        }
    }
}

/// Generic error response from the API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}

/// Generic success response with ID and message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteResponse {
    pub id: serde_json::Value,
    pub success: bool,
    pub message: String,
}

/// Simple success response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuccessResponse {
    pub success: bool,
}
