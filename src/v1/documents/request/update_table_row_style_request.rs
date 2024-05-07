use crate::v1::documents::{request::Location, TableRowStyle};

/// <https://developers.google.com/docs/api/reference/rest/v1/documents/request#updatetablerowstylerequest>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTableRowStyleRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_start_location: Option<Location>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_indices: Option<Vec<usize>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_row_style: Option<TableRowStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<String>,
}
