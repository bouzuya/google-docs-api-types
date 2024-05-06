mod bullet_glyph_preset;
mod create_named_range_request;
mod create_paragraph_bullets_request;
mod delete_content_range_request;
mod delete_named_range_request;
mod delete_named_range_request_named_range_reference;
mod delete_paragraph_bullets_request;
mod delete_positioned_object_request;
mod delete_table_column_request;
mod delete_table_row_request;
mod end_of_segument_location;
mod insert_inline_image_request;
mod insert_inline_image_request_insertion_location;
mod insert_page_break_request;
mod insert_page_break_request_insertion_location;
mod insert_table_column_request;
mod insert_table_request;
mod insert_table_request_insertion_location;
mod insert_table_row_request;
mod insert_text_request;
mod insert_text_request_insertion_location;
mod location;
mod replace_all_text_request;
mod replace_all_text_request_criteria;
mod request_request;
mod substring_match_criteria;
mod table_cell_location;
mod table_range;
mod update_paragraph_style_request;
mod update_paragraph_style_request_insertion_loccation;
mod update_table_cell_style_request;
mod update_table_cell_style_request_cells;
mod update_table_column_properties_request;
mod update_text_style_request;
mod update_text_style_request_insertion_location;

pub use self::bullet_glyph_preset::BulletGlyphPreset;
pub use self::create_named_range_request::CreateNamedRangeRequest;
pub use self::create_paragraph_bullets_request::CreateParagraphBulletsRequest;
pub use self::delete_content_range_request::DeleteContentRangeRequest;
pub use self::delete_named_range_request::DeleteNamedRangeRequest;
pub use self::delete_named_range_request_named_range_reference::DeleteNamedRangeRequestNamedRangeReference;
pub use self::delete_paragraph_bullets_request::DeleteParagraphBulletsRequest;
pub use self::delete_positioned_object_request::DeletePositionedObjectRequest;
pub use self::delete_table_column_request::DeleteTableColumnRequest;
pub use self::delete_table_row_request::DeleteTableRowRequest;
pub use self::end_of_segument_location::EndOfSegmentLocation;
pub use self::insert_inline_image_request::InsertInlineImageRequest;
pub use self::insert_inline_image_request_insertion_location::InsertInlineImageRequestInsertionLocation;
pub use self::insert_page_break_request::InsertPageBreakRequest;
pub use self::insert_page_break_request_insertion_location::InsertPageBreakRequestInsertionLocation;
pub use self::insert_table_column_request::InsertTableColumnRequest;
pub use self::insert_table_request::InsertTableRequest;
pub use self::insert_table_request_insertion_location::InsertTableRequestInsertionLocation;
pub use self::insert_table_row_request::InsertTableRowRequest;
pub use self::insert_text_request::InsertTextRequest;
pub use self::insert_text_request_insertion_location::InsertTextRequestInsertionLocation;
pub use self::location::Location;
pub use self::replace_all_text_request::ReplaceAllTextRequest;
pub use self::replace_all_text_request_criteria::ReplaceAllTextRequestCriteria;
pub use self::request_request::RequestRequest;
pub use self::substring_match_criteria::SubstringMatchCriteria;
pub use self::table_cell_location::TableCellLocation;
pub use self::table_range::TableRange;
pub use self::update_paragraph_style_request::UpdateParagraphStyleRequest;
pub use self::update_paragraph_style_request_insertion_loccation::UpdateParagraphStyleRequestInsertionLocation;
pub use self::update_table_cell_style_request::UpdateTableCellStyleRequest;
pub use self::update_table_cell_style_request_cells::UpdateTableCellStyleRequestCells;
pub use self::update_table_column_properties_request::UpdateTableColumnPropertiesRequest;
pub use self::update_text_style_request::UpdateTextStyleRequest;
pub use self::update_text_style_request_insertion_location::UpdateTextStyleRequestInsertionLocation;

/// <https://developers.google.com/docs/api/reference/rest/v1/documents/request#request>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    #[serde(flatten)]
    pub request: Option<RequestRequest>,
}
