use crate::v1::documents::request::InsertTextRequestInsertionLocation;

#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InsertTextRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub insertion_location: Option<InsertTextRequestInsertionLocation>,
}
