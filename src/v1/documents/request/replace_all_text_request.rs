use crate::v1::documents::request::ReplaceAllTextRequestCriteria;

/// <https://developers.google.com/docs/api/reference/rest/v1/documents/request#replacealltextrequest>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceAllTextRequest {
    pub replace_text: Option<String>,
    #[serde(flatten)]
    pub criteria: Option<ReplaceAllTextRequestCriteria>,
}
