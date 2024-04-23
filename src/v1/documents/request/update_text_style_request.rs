use crate::v1::documents::request::UpdateTextStyleRequestInsertionLocation;
use crate::v1::documents::TextStyle;

/// <https://developers.google.com/docs/api/reference/rest/v1/documents/request#updatetextstylerequest>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTextStyleRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_style: Option<TextStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<String>,
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub insertion_location: Option<UpdateTextStyleRequestInsertionLocation>,
}
