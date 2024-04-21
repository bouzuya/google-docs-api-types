#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    pub segment_id: Option<String>,
    pub index: Option<usize>,
}
