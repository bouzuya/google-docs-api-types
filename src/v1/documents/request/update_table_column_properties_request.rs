use crate::v1::documents::{request::Location, TableColumnProperties};

/// <https://developers.google.com/docs/api/reference/rest/v1/documents/request#updatetablecolumnpropertiesrequest>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTableColumnPropertiesRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_start_location: Option<Location>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_indices: Option<Vec<usize>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_column_properties: Option<TableColumnProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<String>,
}
