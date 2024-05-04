use crate::v1::documents::request::TableCellLocation;

/// <https://developers.google.com/docs/api/reference/rest/v1/documents/request#deletetablecolumnrequest>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteTableColumnRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_cell_location: Option<TableCellLocation>,
}
