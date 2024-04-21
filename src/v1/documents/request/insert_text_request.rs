use crate::v1::documents::request::InsertTextRequestInsertionLocation;

#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InsertTextRequest {
    pub text: Option<String>,
    #[serde(flatten)]
    pub insertion_location: Option<InsertTextRequestInsertionLocation>,
}
