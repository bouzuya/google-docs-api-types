use crate::v1::documents::request::HeaderFooterType;
use crate::v1::documents::request::Location;

/// <https://developers.google.com/docs/api/reference/rest/v1/documents/request#createfooterrequest>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateFooterRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<HeaderFooterType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub section_break_location: Option<Location>,
}
