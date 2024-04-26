use crate::v1::documents::request::delete_named_range_request_named_range_reference::DeleteNamedRangeRequestNamedRangeReference;

/// <https://developers.google.com/docs/api/reference/rest/v1/documents/request#deletenamedrangerequest>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteNamedRangeRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub named_range_reference: Option<DeleteNamedRangeRequestNamedRangeReference>,
}
