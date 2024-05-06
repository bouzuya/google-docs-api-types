use crate::v1::documents::{request::UpdateTableCellStyleRequestCells, TableCellStyle};

/// <https://developers.google.com/docs/api/reference/rest/v1/documents/request#updatetablecellstylerequest>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTableCellStyleRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_cell_style: Option<TableCellStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<String>,
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub cells: Option<UpdateTableCellStyleRequestCells>,
}
