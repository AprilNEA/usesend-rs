use serde_json;
use usesend_api::types::*;
use usesend_api::types::domain::*;
use usesend_api::types::email::*;
use usesend_api::types::campaign::*;

#[test]
fn string_or_vec_single_serialize() {
    let v = StringOrVec::Single("hello@test.com".to_string());
    assert_eq!(serde_json::to_string(&v).unwrap(), r#""hello@test.com""#);
}

#[test]
fn string_or_vec_multiple_serialize() {
    let v = StringOrVec::Multiple(vec!["a@test.com".into(), "b@test.com".into()]);
    assert_eq!(serde_json::to_string(&v).unwrap(), r#"["a@test.com","b@test.com"]"#);
}

#[test]
fn string_or_vec_single_deserialize() {
    let v: StringOrVec = serde_json::from_str(r#""hello@test.com""#).unwrap();
    assert_eq!(v, StringOrVec::Single("hello@test.com".to_string()));
}

#[test]
fn string_or_vec_multiple_deserialize() {
    let v: StringOrVec = serde_json::from_str(r#"["a@test.com","b@test.com"]"#).unwrap();
    assert_eq!(v, StringOrVec::Multiple(vec!["a@test.com".into(), "b@test.com".into()]));
}

#[test]
fn string_or_vec_from_str() {
    let v: StringOrVec = "test@test.com".into();
    assert_eq!(v, StringOrVec::Single("test@test.com".to_string()));
}

#[test]
fn string_or_vec_from_vec() {
    let v: StringOrVec = vec!["a@test.com".to_string()].into();
    assert_eq!(v, StringOrVec::Multiple(vec!["a@test.com".into()]));
}

#[test]
fn domain_id_display() {
    let id = DomainId(42);
    assert_eq!(format!("{id}"), "42");
}

#[test]
fn email_id_deref() {
    let id = EmailId::from("abc123");
    assert_eq!(&*id, "abc123");
    assert_eq!(id.as_ref(), "abc123");
}

#[test]
fn domain_id_serde_roundtrip() {
    let id = DomainId(99);
    let json = serde_json::to_string(&id).unwrap();
    assert_eq!(json, "99");
    let back: DomainId = serde_json::from_str(&json).unwrap();
    assert_eq!(back, id);
}

#[test]
fn email_id_serde_roundtrip() {
    let id = EmailId::from("test-id-123");
    let json = serde_json::to_string(&id).unwrap();
    assert_eq!(json, r#""test-id-123""#);
    let back: EmailId = serde_json::from_str(&json).unwrap();
    assert_eq!(back, id);
}

#[test]
fn domain_status_serde() {
    let s = DomainStatus::TemporaryFailure;
    let json = serde_json::to_string(&s).unwrap();
    assert_eq!(json, r#""TEMPORARY_FAILURE""#);
    let back: DomainStatus = serde_json::from_str(&json).unwrap();
    assert_eq!(back, s);
}

#[test]
fn dns_record_type_serde() {
    let t = DnsRecordType::MX;
    let json = serde_json::to_string(&t).unwrap();
    assert_eq!(json, r#""MX""#);
}

#[test]
fn email_event_status_serde() {
    let s = EmailEventStatus::DeliveryDelayed;
    let json = serde_json::to_string(&s).unwrap();
    assert_eq!(json, r#""DELIVERY_DELAYED""#);
}

#[test]
fn campaign_status_serde() {
    let s = CampaignStatus::InProgress;
    let json = serde_json::to_string(&s).unwrap();
    assert_eq!(json, r#""IN_PROGRESS""#);
}

#[test]
fn list_emails_params_default() {
    let p = ListEmailsParams::default();
    assert!(p.page.is_none());
    assert!(p.limit.is_none());
}

#[test]
fn error_response_deserialize() {
    let json = r#"{"error":"Not found"}"#;
    let e: ErrorResponse = serde_json::from_str(json).unwrap();
    assert_eq!(e.error, "Not found");
}

#[test]
fn send_email_response_deserialize() {
    let json = r#"{"emailId":"abc-123"}"#;
    let r: SendEmailResponse = serde_json::from_str(json).unwrap();
    assert_eq!(&*r.email_id, "abc-123");
}

#[test]
fn nullable_latest_status_deserialize() {
    // latestStatus can be null
    let json = r#"{"id":"1","to":"a@b.com","from":"x@y.com","subject":"s","html":null,"text":null,"createdAt":"2024","updatedAt":"2024","latestStatus":null,"scheduledAt":null,"domainId":null}"#;
    let item: EmailListItem = serde_json::from_str(json).unwrap();
    assert!(item.latest_status.is_none());
}
