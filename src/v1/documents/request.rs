mod bullet_glyph_preset;
mod create_named_range_request;
mod create_paragraph_bullets_request;
mod delete_content_range_request;
mod delete_named_range_request;
mod delete_named_range_request_named_range_reference;
mod delete_paragraph_bullets_request;
mod end_of_segument_location;
mod insert_inline_image_request;
mod insert_inline_image_request_insertion_location;
mod insert_table_request;
mod insert_table_request_insertion_location;
mod insert_text_request;
mod insert_text_request_insertion_location;
mod location;
mod replace_all_text_request;
mod replace_all_text_request_criteria;
mod request_request;
mod substring_match_criteria;
mod update_paragraph_style_request;
mod update_paragraph_style_request_insertion_loccation;
mod update_text_style_request;
mod update_text_style_request_insertion_location;

pub use self::bullet_glyph_preset::BulletGlyphPreset;
pub use self::create_named_range_request::CreateNamedRangeRequest;
pub use self::create_paragraph_bullets_request::CreateParagraphBulletsRequest;
pub use self::delete_content_range_request::DeleteContentRangeRequest;
pub use self::delete_named_range_request::DeleteNamedRangeRequest;
pub use self::delete_named_range_request_named_range_reference::DeleteNamedRangeRequestNamedRangeReference;
pub use self::delete_paragraph_bullets_request::DeleteParagraphBulletsRequest;
pub use self::end_of_segument_location::EndOfSegmentLocation;
pub use self::insert_inline_image_request::InsertInlineImageRequest;
pub use self::insert_inline_image_request_insertion_location::InsertInlineImageRequestInsertionLocation;
pub use self::insert_table_request::InsertTableRequest;
pub use self::insert_table_request_insertion_location::InsertTableRequestInsertionLocation;
pub use self::insert_text_request::InsertTextRequest;
pub use self::insert_text_request_insertion_location::InsertTextRequestInsertionLocation;
pub use self::location::Location;
pub use self::replace_all_text_request::ReplaceAllTextRequest;
pub use self::replace_all_text_request_criteria::ReplaceAllTextRequestCriteria;
pub use self::request_request::RequestRequest;
pub use self::substring_match_criteria::SubstringMatchCriteria;
pub use self::update_paragraph_style_request::UpdateParagraphStyleRequest;
pub use self::update_paragraph_style_request_insertion_loccation::UpdateParagraphStyleRequestInsertionLocation;
pub use self::update_text_style_request::UpdateTextStyleRequest;
pub use self::update_text_style_request_insertion_location::UpdateTextStyleRequestInsertionLocation;

/// <https://developers.google.com/docs/api/reference/rest/v1/documents/request#request>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    #[serde(flatten)]
    pub request: Option<RequestRequest>,
}
