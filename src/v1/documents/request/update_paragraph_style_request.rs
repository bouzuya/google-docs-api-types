use crate::v1::documents::request::UpdateParagraphStyleRequestInsertionLocation;

/// <https://developers.google.com/docs/api/reference/rest/v1/documents/request#updateparagraphstylerequest>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateParagraphStyleRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paragraph_style: Option<crate::v1::documents::ParagraphStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insertion_location: Option<UpdateParagraphStyleRequestInsertionLocation>,
}
