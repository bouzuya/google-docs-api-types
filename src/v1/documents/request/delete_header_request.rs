/// <https://developers.google.com/docs/api/reference/rest/v1/documents/request#deleteheaderrequest>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteHeaderRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header_id: Option<String>,
}
