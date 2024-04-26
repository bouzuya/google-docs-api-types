/// <https://developers.google.com/docs/api/reference/rest/v1/documents/request#deletenamedrangerequest>
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum DeleteNamedRangeRequestNamedRangeReference {
    NamedRangeId(String),
    Name(String),
}
