use crate::v1::documents::request::CreateNamedRangeRequest;
use crate::v1::documents::request::CreateParagraphBulletsRequest;
use crate::v1::documents::request::DeleteContentRangeRequest;
use crate::v1::documents::request::DeleteNamedRangeRequest;
use crate::v1::documents::request::DeleteParagraphBulletsRequest;
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
    CreateParagraphBullets(CreateParagraphBulletsRequest),
    DeleteParagraphBullets(DeleteParagraphBulletsRequest),
    CreateNamedRange(CreateNamedRangeRequest),
    DeleteNamedRange(DeleteNamedRangeRequest),
    // TODO: ...
    DeleteContentRange(DeleteContentRangeRequest),
    InsertInlineImage(InsertInlineImageRequest),
    // TODO: ...
}
