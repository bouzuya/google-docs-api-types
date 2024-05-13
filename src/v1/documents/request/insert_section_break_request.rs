use crate::v1::documents::request::InsertSectionBreakRequestInsertionLocation;
use crate::v1::documents::SectionType;

/// <https://developers.google.com/docs/api/reference/rest/v1/documents/request#insertsectionbreakrequest>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InsertSectionBreakRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub section_type: Option<SectionType>,
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub insertion_location: Option<InsertSectionBreakRequestInsertionLocation>,
}
