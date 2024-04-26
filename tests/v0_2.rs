use google_docs_api_types::v1::documents::request::CreateNamedRangeRequest;

#[test]
fn test_create_paragraph_bullets_request() {
    // Adds CreateParagraphBulletsRequest and BulletGlyphPreset
    use google_docs_api_types::v1::documents::request::BulletGlyphPreset;
    use google_docs_api_types::v1::documents::request::CreateParagraphBulletsRequest;
    use google_docs_api_types::v1::documents::request::Request;
    use google_docs_api_types::v1::documents::request::RequestRequest;
    use google_docs_api_types::v1::documents::Range;
    let _ = Request {
        request: Some(RequestRequest::CreateParagraphBullets(
            CreateParagraphBulletsRequest {
                range: Some(Range::default()),
                bullet_preset: Some(BulletGlyphPreset::default()),
            },
        )),
    };
}

#[test]
fn test_create_named_range_request() {
    // Adds CreateNamedRangeRequest
    use google_docs_api_types::v1::documents::request::Request;
    use google_docs_api_types::v1::documents::request::RequestRequest;
    use google_docs_api_types::v1::documents::Range;
    let _ = Request {
        request: Some(RequestRequest::CreateNamedRange(CreateNamedRangeRequest {
            name: Some(String::default()),
            range: Some(Range::default()),
        })),
    };
}
#[test]
fn test_delete_named_range_request() {
    // Adds DeleteNamedRangeRequest and DeleteNamedRangeRequestNamedRangeReference
    use google_docs_api_types::v1::documents::request::DeleteNamedRangeRequest;
    use google_docs_api_types::v1::documents::request::DeleteNamedRangeRequestNamedRangeReference;
    use google_docs_api_types::v1::documents::request::Request;
    use google_docs_api_types::v1::documents::request::RequestRequest;
    let _ = Request {
        request: Some(RequestRequest::DeleteNamedRange(DeleteNamedRangeRequest {
            named_range_reference: Some(DeleteNamedRangeRequestNamedRangeReference::NamedRangeId(
                String::default(),
            )),
        })),
    };
}

#[test]
fn test_delete_paragraph_bullets_request() {
    // Adds DeleteParagraphBulletsRequest
    use google_docs_api_types::v1::documents::request::DeleteParagraphBulletsRequest;
    use google_docs_api_types::v1::documents::request::Request;
    use google_docs_api_types::v1::documents::request::RequestRequest;
    use google_docs_api_types::v1::documents::Range;
    let _ = Request {
        request: Some(RequestRequest::DeleteParagraphBullets(
            DeleteParagraphBulletsRequest {
                range: Some(Range::default()),
            },
        )),
    };
}

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
