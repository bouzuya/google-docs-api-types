use crate::v1::documents::Range;

#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteContentRangeRequest {
    pub range: Option<Range>,
}
