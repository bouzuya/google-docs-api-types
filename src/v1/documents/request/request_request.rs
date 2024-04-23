use crate::v1::documents::request::DeleteContentRangeRequest;
use crate::v1::documents::request::InsertInlineImageRequest;
use crate::v1::documents::request::InsertTextRequest;
use crate::v1::documents::request::ReplaceAllTextRequest;
use crate::v1::documents::request::UpdateTextStyleRequest;

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum RequestRequest {
    ReplaceAllText(ReplaceAllTextRequest),
    InsertText(InsertTextRequest),
    UpdateTextStyle(UpdateTextStyleRequest),
    // TODO: ...
    DeleteContentRange(DeleteContentRangeRequest),
    InsertInlineImage(InsertInlineImageRequest),
    // TODO: ...
}
