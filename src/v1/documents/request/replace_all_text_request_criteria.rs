use crate::v1::documents::request::SubstringMatchCriteria;

/// <https://developers.google.com/docs/api/reference/rest/v1/documents/request#replacealltextrequest>
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ReplaceAllTextRequestCriteria {
    ContainsText(SubstringMatchCriteria),
}
