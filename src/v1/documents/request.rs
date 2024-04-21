mod delete_content_range_request;
mod end_of_segument_location;
mod insert_inline_image_request;
mod insert_inline_image_request_insertion_location;
mod insert_text_request;
mod insert_text_request_insertion_location;
mod location;
mod replace_all_text_request;
mod replace_all_text_request_criteria;
mod request_request;
mod substring_match_criteria;

pub use self::delete_content_range_request::DeleteContentRangeRequest;
pub use self::end_of_segument_location::EndOfSegmentLocation;
pub use self::insert_inline_image_request::InsertInlineImageRequest;
pub use self::insert_inline_image_request_insertion_location::InsertInlineImageRequestInsertionLocation;
pub use self::insert_text_request::InsertTextRequest;
pub use self::insert_text_request_insertion_location::InsertTextRequestInsertionLocation;
pub use self::location::Location;
pub use self::replace_all_text_request::ReplaceAllTextRequest;
pub use self::replace_all_text_request_criteria::ReplaceAllTextRequestCriteria;
pub use self::request_request::RequestRequest;
pub use self::substring_match_criteria::SubstringMatchCriteria;

/// <https://developers.google.com/docs/api/reference/rest/v1/documents/request#request>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    #[serde(flatten)]
    pub request: Option<RequestRequest>,
}