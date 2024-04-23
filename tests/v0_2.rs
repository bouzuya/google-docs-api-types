#[test]
fn test_update_text_style_request() {
    // Adds UpdateTextStyleRequest and UpdateTextStyleRequestInsertionLocation
    use google_docs_api_types::v1::documents::request::Request;
    use google_docs_api_types::v1::documents::request::RequestRequest;
    use google_docs_api_types::v1::documents::request::UpdateTextStyleRequest;
    use google_docs_api_types::v1::documents::request::UpdateTextStyleRequestInsertionLocation;
    use google_docs_api_types::v1::documents::Range;
    use google_docs_api_types::v1::documents::TextStyle;
    let _ = Request {
        request: Some(RequestRequest::UpdateTextStyle(UpdateTextStyleRequest {
            text_style: Some(TextStyle::default()),
            fields: Some(String::default()),
            insertion_location: Some(UpdateTextStyleRequestInsertionLocation::Range(
                Range::default(),
            )),
        })),
    };
}
