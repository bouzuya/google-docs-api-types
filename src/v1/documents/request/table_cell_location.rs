use crate::v1::documents::request::Location;

/// <https://developers.google.com/docs/api/reference/rest/v1/documents/request#tablecelllocation>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TableCellLocation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_start_location: Option<Location>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_index: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_index: Option<usize>,
}
