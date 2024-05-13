use crate::v1::documents::request::ReplaceNamedRangeContentRequestNamedRangeReference;
use crate::v1::documents::request::ReplaceNamedRangeContentRequestReplacementContent;

/// <https://developers.google.com/docs/api/reference/rest/v1/documents/request#replacenamedrangecontentrequest>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceNamedRangeContentRequest {
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub replacement_content: Option<ReplaceNamedRangeContentRequestReplacementContent>,
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub named_range_reference: Option<ReplaceNamedRangeContentRequestNamedRangeReference>,
}
