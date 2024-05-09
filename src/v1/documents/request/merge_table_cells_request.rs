use crate::v1::documents::request::TableRange;

/// <https://developers.google.com/docs/api/reference/rest/v1/documents/request#mergetablecellsrequest>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MergeTableCellsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_range: Option<TableRange>,
}
