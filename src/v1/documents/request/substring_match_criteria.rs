/// <https://developers.google.com/docs/api/reference/rest/v1/documents/request#substringmatchcriteria>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubstringMatchCriteria {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_case: Option<bool>,
}
