#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EndOfSegmentLocation {
    pub segment_id: Option<String>,
}
