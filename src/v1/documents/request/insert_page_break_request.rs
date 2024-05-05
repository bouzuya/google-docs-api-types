use crate::v1::documents::request::InsertPageBreakRequestInsertionLocation;

/// <https://developers.google.com/docs/api/reference/rest/v1/documents/request#insertpagebreakrequest>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InsertPageBreakRequest {
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub insertion_location: Option<InsertPageBreakRequestInsertionLocation>,
}
