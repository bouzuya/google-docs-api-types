use crate::v1::documents::request::TableCellLocation;

/// <https://developers.google.com/docs/api/reference/rest/v1/documents/request#inserttablecolumnrequest>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InsertTableColumnRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_cell_location: Option<TableCellLocation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insert_right: Option<bool>,
}
