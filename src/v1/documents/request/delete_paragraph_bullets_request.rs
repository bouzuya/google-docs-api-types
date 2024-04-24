use crate::v1::documents::Range;

/// <https://developers.google.com/docs/api/reference/rest/v1/documents/request#deleteparagraphbulletsrequest>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteParagraphBulletsRequest {
    pub range: Option<Range>,
}
