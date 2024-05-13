use crate::v1::documents::request::InsertInlineImageRequestInsertionLocation;
use crate::v1::documents::Size;

/// <https://developers.google.com/docs/api/reference/rest/v1/documents/request#insertinlineimagerequest>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InsertInlineImageRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_size: Option<Size>,
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub insertion_location: Option<InsertInlineImageRequestInsertionLocation>,
}
