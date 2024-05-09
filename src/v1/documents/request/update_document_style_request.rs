use crate::v1::documents::DocumentStyle;

/// <https://developers.google.com/docs/api/reference/rest/v1/documents/request#updatedocumentstylerequest>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateDocumentStyleRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_style: Option<DocumentStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<String>,
}
