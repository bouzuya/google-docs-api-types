use crate::v1::documents::request::EndOfSegmentLocation;
use crate::v1::documents::request::Location;

/// <https://developers.google.com/docs/api/reference/rest/v1/documents/request#insertsectionbreakrequest>
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum InsertSectionBreakRequestInsertionLocation {
    Location(Location),
    EndOfSegmentLocation(EndOfSegmentLocation),
}
