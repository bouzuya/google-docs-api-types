use crate::v1::documents::request::ImageReplaceMethod;

/// <https://developers.google.com/docs/api/reference/rest/v1/documents/request#replaceimagerequest>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceImageRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_object_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub image_replace_method: Option<ImageReplaceMethod>,
}
