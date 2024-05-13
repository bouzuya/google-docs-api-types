/// <https://developers.google.com/docs/api/reference/rest/v1/documents/request#deletefooterrequest>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteFooterRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer_id: Option<String>,
}
