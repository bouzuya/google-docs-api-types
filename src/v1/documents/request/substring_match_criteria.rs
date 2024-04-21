/// <https://developers.google.com/docs/api/reference/rest/v1/documents/request#substringmatchcriteria>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubstringMatchCriteria {
    pub text: Option<String>,
    pub match_case: Option<bool>,
}
