use crate::v1::documents::request::Location;

#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PinTableHeaderRowsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_start_location: Option<Location>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinned_header_rows_count: Option<usize>,
}
