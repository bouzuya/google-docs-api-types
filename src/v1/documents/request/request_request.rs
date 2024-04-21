use crate::v1::documents::request::DeleteContentRangeRequest;
use crate::v1::documents::request::InsertInlineImageRequest;
use crate::v1::documents::request::InsertTextRequest;
use crate::v1::documents::request::ReplaceAllTextRequest;

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum RequestRequest {
    ReplaceAllText(ReplaceAllTextRequest),
    InsertText(InsertTextRequest),
    // TODO: ...
    DeleteContentRange(DeleteContentRangeRequest),
    InsertInlineImage(InsertInlineImageRequest),
    // TODO: ...
}
