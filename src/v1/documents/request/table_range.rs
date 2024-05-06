use crate::v1::documents::request::TableCellLocation;

/// <https://developers.google.com/docs/api/reference/rest/v1/documents/request#tablerange>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TableRange {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_cell_location: Option<TableCellLocation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_span: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_span: Option<usize>,
}
