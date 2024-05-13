use crate::v1::documents::{Range, SectionStyle};

/// <https://developers.google.com/docs/api/reference/rest/v1/documents/request#updatesectionstylerequest>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSectionStyleRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<Range>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub section_style: Option<SectionStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<String>,
}
