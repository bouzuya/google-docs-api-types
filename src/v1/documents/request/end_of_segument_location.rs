#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EndOfSegmentLocation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_id: Option<String>,
}
