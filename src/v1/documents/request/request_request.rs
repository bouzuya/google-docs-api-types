use crate::v1::documents::request::CreateFooterRequest;
use crate::v1::documents::request::CreateFootnoteRequest;
use crate::v1::documents::request::CreateHeaderRequest;
use crate::v1::documents::request::CreateNamedRangeRequest;
use crate::v1::documents::request::CreateParagraphBulletsRequest;
use crate::v1::documents::request::DeleteContentRangeRequest;
use crate::v1::documents::request::DeleteFooterRequest;
use crate::v1::documents::request::DeleteHeaderRequest;
use crate::v1::documents::request::DeleteNamedRangeRequest;
use crate::v1::documents::request::DeleteParagraphBulletsRequest;
use crate::v1::documents::request::DeletePositionedObjectRequest;
use crate::v1::documents::request::DeleteTableColumnRequest;
use crate::v1::documents::request::DeleteTableRowRequest;
use crate::v1::documents::request::InsertInlineImageRequest;
use crate::v1::documents::request::InsertPageBreakRequest;
use crate::v1::documents::request::InsertSectionBreakRequest;
use crate::v1::documents::request::InsertTableColumnRequest;
use crate::v1::documents::request::InsertTableRequest;
use crate::v1::documents::request::InsertTableRowRequest;
use crate::v1::documents::request::InsertTextRequest;
use crate::v1::documents::request::MergeTableCellsRequest;
use crate::v1::documents::request::PinTableHeaderRowsRequest;
use crate::v1::documents::request::ReplaceAllTextRequest;
use crate::v1::documents::request::ReplaceImageRequest;
use crate::v1::documents::request::ReplaceNamedRangeContentRequest;
use crate::v1::documents::request::UnmergeTableCellsRequest;
use crate::v1::documents::request::UpdateDocumentStyleRequest;
use crate::v1::documents::request::UpdateParagraphStyleRequest;
use crate::v1::documents::request::UpdateSectionStyleRequest;
use crate::v1::documents::request::UpdateTableCellStyleRequest;
use crate::v1::documents::request::UpdateTableColumnPropertiesRequest;
use crate::v1::documents::request::UpdateTableRowStyleRequest;
use crate::v1::documents::request::UpdateTextStyleRequest;

#[allow(clippy::large_enum_variant)]
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
    UpdateParagraphStyle(UpdateParagraphStyleRequest),
    DeleteContentRange(DeleteContentRangeRequest),
    InsertInlineImage(InsertInlineImageRequest),
    InsertTable(InsertTableRequest),
    InsertTableRow(InsertTableRowRequest),
    InsertTableColumn(InsertTableColumnRequest),
    DeleteTableRow(DeleteTableRowRequest),
    DeleteTableColumn(DeleteTableColumnRequest),
    InsertPageBreak(InsertPageBreakRequest),
    DeletePositionedObject(DeletePositionedObjectRequest),
    UpdateTableColumnProperties(UpdateTableColumnPropertiesRequest),
    UpdateTableCellStyle(UpdateTableCellStyleRequest),
    UpdateTableRowStyle(UpdateTableRowStyleRequest),
    ReplaceImage(ReplaceImageRequest),
    UpdateDocumentStyle(UpdateDocumentStyleRequest),
    MergeTableCells(MergeTableCellsRequest),
    UnmergeTableCells(UnmergeTableCellsRequest),
    CreateHeader(CreateHeaderRequest),
    CreateFooter(CreateFooterRequest),
    CreateFootnote(CreateFootnoteRequest),
    ReplaceNamedRangeContent(ReplaceNamedRangeContentRequest),
    UpdateSectionStyle(UpdateSectionStyleRequest),
    InsertSectionBreak(InsertSectionBreakRequest),
    DeleteHeader(DeleteHeaderRequest),
    DeleteFooter(DeleteFooterRequest),
    PinTableHeaderRows(PinTableHeaderRowsRequest),
}
