use crate::v1::documents::Range;

#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteContentRangeRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<Range>,
}
