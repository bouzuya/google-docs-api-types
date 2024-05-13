/// <https://developers.google.com/docs/api/reference/rest/v1/documents/request#replacenamedrangecontentrequest>
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ReplaceNamedRangeContentRequestReplacementContent {
    Text(String),
}
