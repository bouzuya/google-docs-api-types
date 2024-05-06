use crate::v1::documents::request::Location;
use crate::v1::documents::request::TableRange;

/// <https://developers.google.com/docs/api/reference/rest/v1/documents/request#updatetablecellstylerequest>
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum UpdateTableCellStyleRequestCells {
    TableRange(TableRange),
    TableStartLocation(Location),
}
