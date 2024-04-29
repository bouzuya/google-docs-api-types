use crate::v1::documents::request::InsertTableRequestInsertionLocation;

/// <https://developers.google.com/docs/api/reference/rest/v1/documents/request#inserttablerequest>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InsertTableRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rows: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub columns: Option<usize>,
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub insertion_location: Option<InsertTableRequestInsertionLocation>,
}
