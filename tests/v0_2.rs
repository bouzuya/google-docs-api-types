#[test]
fn test_create_footer_request() {
    // Adds CreateFooterRequest
    use google_docs_api_types::v1::documents::request::CreateFooterRequest;
    use google_docs_api_types::v1::documents::request::HeaderFooterType;
    use google_docs_api_types::v1::documents::request::Location;
    use google_docs_api_types::v1::documents::request::Request;
    use google_docs_api_types::v1::documents::request::RequestRequest;
    let _ = Request {
        request: Some(RequestRequest::CreateFooter(CreateFooterRequest {
            r#type: Some(HeaderFooterType::default()),
            section_break_location: Some(Location::default()),
        })),
    };
}

#[test]
fn test_create_footnote_request() {
    // Adds CreateFootnoteRequest and CreateFootnoteRequestFootnoteReferenceLocation
    use google_docs_api_types::v1::documents::request::CreateFootnoteRequest;
    use google_docs_api_types::v1::documents::request::CreateFootnoteRequestFootnoteReferenceLocation;
    use google_docs_api_types::v1::documents::request::Location;
    use google_docs_api_types::v1::documents::request::Request;
    use google_docs_api_types::v1::documents::request::RequestRequest;
    let _ = Request {
        request: Some(RequestRequest::CreateFootnote(CreateFootnoteRequest {
            footnote_reference_location: Some(
                CreateFootnoteRequestFootnoteReferenceLocation::Location(Location::default()),
            ),
        })),
    };
}

#[test]
fn test_create_header_request() {
    // Adds CreateHeaderRequest and HeaderFooterType
    use google_docs_api_types::v1::documents::request::CreateHeaderRequest;
    use google_docs_api_types::v1::documents::request::HeaderFooterType;
    use google_docs_api_types::v1::documents::request::Location;
    use google_docs_api_types::v1::documents::request::Request;
    use google_docs_api_types::v1::documents::request::RequestRequest;
    let _ = Request {
        request: Some(RequestRequest::CreateHeader(CreateHeaderRequest {
            r#type: Some(HeaderFooterType::default()),
            section_break_location: Some(Location::default()),
        })),
    };
}

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
    use google_docs_api_types::v1::documents::request::CreateNamedRangeRequest;
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
fn test_delete_positioned_object_request() {
    // Adds DeletePositionedObjectRequest
    use google_docs_api_types::v1::documents::request::DeletePositionedObjectRequest;
    use google_docs_api_types::v1::documents::request::Request;
    use google_docs_api_types::v1::documents::request::RequestRequest;
    let _ = Request {
        request: Some(RequestRequest::DeletePositionedObject(
            DeletePositionedObjectRequest {
                object_id: Some(String::default()),
            },
        )),
    };
}

#[test]
fn test_delete_table_column_request() {
    // Adds DeleteTableColumnRequest
    use google_docs_api_types::v1::documents::request::DeleteTableColumnRequest;
    use google_docs_api_types::v1::documents::request::Location;
    use google_docs_api_types::v1::documents::request::Request;
    use google_docs_api_types::v1::documents::request::RequestRequest;
    use google_docs_api_types::v1::documents::request::TableCellLocation;
    let _ = Request {
        request: Some(RequestRequest::DeleteTableColumn(
            DeleteTableColumnRequest {
                table_cell_location: Some(TableCellLocation {
                    table_start_location: Some(Location::default()),
                    row_index: Some(usize::default()),
                    column_index: Some(usize::default()),
                }),
            },
        )),
    };
}

#[test]
fn test_delete_table_row_request() {
    // Adds DeleteTableRowRequest
    use google_docs_api_types::v1::documents::request::DeleteTableRowRequest;
    use google_docs_api_types::v1::documents::request::Location;
    use google_docs_api_types::v1::documents::request::Request;
    use google_docs_api_types::v1::documents::request::RequestRequest;
    use google_docs_api_types::v1::documents::request::TableCellLocation;
    let _ = Request {
        request: Some(RequestRequest::DeleteTableRow(DeleteTableRowRequest {
            table_cell_location: Some(TableCellLocation {
                table_start_location: Some(Location::default()),
                row_index: Some(usize::default()),
                column_index: Some(usize::default()),
            }),
        })),
    };
}

#[test]
fn test_insert_page_break_request() {
    // Adds InsertPageBreakRequest
    use google_docs_api_types::v1::documents::request::InsertPageBreakRequest;
    use google_docs_api_types::v1::documents::request::InsertPageBreakRequestInsertionLocation;
    use google_docs_api_types::v1::documents::request::Location;
    use google_docs_api_types::v1::documents::request::Request;
    use google_docs_api_types::v1::documents::request::RequestRequest;
    let _ = Request {
        request: Some(RequestRequest::InsertPageBreak(InsertPageBreakRequest {
            insertion_location: Some(InsertPageBreakRequestInsertionLocation::Location(
                Location::default(),
            )),
        })),
    };
}

#[test]
fn test_insert_table_column_request() {
    // Adds InsertTableColumnRequest
    use google_docs_api_types::v1::documents::request::InsertTableColumnRequest;
    use google_docs_api_types::v1::documents::request::Location;
    use google_docs_api_types::v1::documents::request::Request;
    use google_docs_api_types::v1::documents::request::RequestRequest;
    use google_docs_api_types::v1::documents::request::TableCellLocation;
    let _ = Request {
        request: Some(RequestRequest::InsertTableColumn(
            InsertTableColumnRequest {
                table_cell_location: Some(TableCellLocation {
                    table_start_location: Some(Location::default()),
                    row_index: Some(usize::default()),
                    column_index: Some(usize::default()),
                }),
                insert_right: Some(bool::default()),
            },
        )),
    };
}

#[test]
fn test_insert_table_request() {
    // Adds InsertTableRequest and InsertTableRequestInsertionLocation
    use google_docs_api_types::v1::documents::request::InsertTableRequest;
    use google_docs_api_types::v1::documents::request::InsertTableRequestInsertionLocation;
    use google_docs_api_types::v1::documents::request::Location;
    use google_docs_api_types::v1::documents::request::Request;
    use google_docs_api_types::v1::documents::request::RequestRequest;
    let _ = Request {
        request: Some(RequestRequest::InsertTable(InsertTableRequest {
            rows: Some(usize::default()),
            columns: Some(usize::default()),
            insertion_location: Some(InsertTableRequestInsertionLocation::Location(
                Location::default(),
            )),
        })),
    };
}

#[test]
fn test_insert_table_row_request() {
    // Adds InsertTableRowRequest and TableCellLocation
    use google_docs_api_types::v1::documents::request::InsertTableRowRequest;
    use google_docs_api_types::v1::documents::request::Location;
    use google_docs_api_types::v1::documents::request::Request;
    use google_docs_api_types::v1::documents::request::RequestRequest;
    use google_docs_api_types::v1::documents::request::TableCellLocation;
    let _ = Request {
        request: Some(RequestRequest::InsertTableRow(InsertTableRowRequest {
            table_cell_location: Some(TableCellLocation {
                table_start_location: Some(Location::default()),
                row_index: Some(usize::default()),
                column_index: Some(usize::default()),
            }),
            insert_below: Some(bool::default()),
        })),
    };
}

#[test]
fn test_merge_table_cells_request() {
    // Adds MergeTableCellsRequest
    use google_docs_api_types::v1::documents::request::MergeTableCellsRequest;
    use google_docs_api_types::v1::documents::request::Request;
    use google_docs_api_types::v1::documents::request::RequestRequest;
    use google_docs_api_types::v1::documents::request::TableRange;
    let _ = Request {
        request: Some(RequestRequest::MergeTableCells(MergeTableCellsRequest {
            table_range: Some(TableRange::default()),
        })),
    };
}

#[test]
fn test_replace_image_request() {
    // Adds ReplaceImageRequest and ImageReplaceMethod
    use google_docs_api_types::v1::documents::request::ImageReplaceMethod;
    use google_docs_api_types::v1::documents::request::ReplaceImageRequest;
    use google_docs_api_types::v1::documents::request::Request;
    use google_docs_api_types::v1::documents::request::RequestRequest;
    let _ = Request {
        request: Some(RequestRequest::ReplaceImage(ReplaceImageRequest {
            image_object_id: Some(String::default()),
            uri: Some(String::default()),
            image_replace_method: Some(ImageReplaceMethod::default()),
        })),
    };
}

#[test]
fn test_unmerge_table_cells_request() {
    // Adds UnmergeTableCellsRequest
    use google_docs_api_types::v1::documents::request::Request;
    use google_docs_api_types::v1::documents::request::RequestRequest;
    use google_docs_api_types::v1::documents::request::TableRange;
    use google_docs_api_types::v1::documents::request::UnmergeTableCellsRequest;
    let _ = Request {
        request: Some(RequestRequest::UnmergeTableCells(
            UnmergeTableCellsRequest {
                table_range: Some(TableRange::default()),
            },
        )),
    };
}

#[test]
fn test_update_document_style_request() {
    // Adds UpdateDocumentStyleRequest
    use google_docs_api_types::v1::documents::request::Request;
    use google_docs_api_types::v1::documents::request::RequestRequest;
    use google_docs_api_types::v1::documents::request::UpdateDocumentStyleRequest;
    use google_docs_api_types::v1::documents::DocumentStyle;
    let _ = Request {
        request: Some(RequestRequest::UpdateDocumentStyle(
            UpdateDocumentStyleRequest {
                document_style: Some(DocumentStyle::default()),
                fields: Some(String::default()),
            },
        )),
    };
}

#[test]
fn test_update_paragraph_style_request() {
    // Adds UpdateParagraphStyleRequest and UpdateParagraphStyleRequestInsertionLocation
    use google_docs_api_types::v1::documents::request::Request;
    use google_docs_api_types::v1::documents::request::RequestRequest;
    use google_docs_api_types::v1::documents::request::UpdateParagraphStyleRequest;
    use google_docs_api_types::v1::documents::request::UpdateParagraphStyleRequestInsertionLocation;
    use google_docs_api_types::v1::documents::ParagraphStyle;
    use google_docs_api_types::v1::documents::Range;
    let _ = Request {
        request: Some(RequestRequest::UpdateParagraphStyle(
            UpdateParagraphStyleRequest {
                paragraph_style: Some(ParagraphStyle::default()),
                fields: Some(String::default()),
                insertion_location: Some(UpdateParagraphStyleRequestInsertionLocation::Range(
                    Range::default(),
                )),
            },
        )),
    };
}

#[test]
fn test_update_table_cell_style_request() {
    // Adds UpdateTableCellStyleRequest and TableRange
    use google_docs_api_types::v1::documents::request::Request;
    use google_docs_api_types::v1::documents::request::RequestRequest;
    use google_docs_api_types::v1::documents::request::TableCellLocation;
    use google_docs_api_types::v1::documents::request::TableRange;
    use google_docs_api_types::v1::documents::request::UpdateTableCellStyleRequest;
    use google_docs_api_types::v1::documents::request::UpdateTableCellStyleRequestCells;
    use google_docs_api_types::v1::documents::TableCellStyle;
    let _ = Request {
        request: Some(RequestRequest::UpdateTableCellStyle(
            UpdateTableCellStyleRequest {
                table_cell_style: Some(TableCellStyle::default()),
                fields: Some(String::default()),
                cells: Some(UpdateTableCellStyleRequestCells::TableRange(TableRange {
                    table_cell_location: Some(TableCellLocation::default()),
                    row_span: Some(usize::default()),
                    column_span: Some(usize::default()),
                })),
            },
        )),
    };
}

#[test]
fn test_update_table_column_properties_request() {
    // Adds UpdateTableColumnPropertiesRequest
    use google_docs_api_types::v1::documents::request::Location;
    use google_docs_api_types::v1::documents::request::Request;
    use google_docs_api_types::v1::documents::request::RequestRequest;
    use google_docs_api_types::v1::documents::request::UpdateTableColumnPropertiesRequest;
    use google_docs_api_types::v1::documents::TableColumnProperties;
    let _ = Request {
        request: Some(RequestRequest::UpdateTableColumnProperties(
            UpdateTableColumnPropertiesRequest {
                table_start_location: Some(Location::default()),
                column_indices: Some(vec![usize::default()]),
                table_column_properties: Some(TableColumnProperties::default()),
                fields: Some(String::default()),
            },
        )),
    };
}

#[test]
fn test_update_table_row_style_request() {
    // Adds UpdateTableRowStyleRequest
    use google_docs_api_types::v1::documents::request::Location;
    use google_docs_api_types::v1::documents::request::Request;
    use google_docs_api_types::v1::documents::request::RequestRequest;
    use google_docs_api_types::v1::documents::request::UpdateTableRowStyleRequest;
    use google_docs_api_types::v1::documents::TableRowStyle;
    let _ = Request {
        request: Some(RequestRequest::UpdateTableRowStyle(
            UpdateTableRowStyleRequest {
                table_start_location: Some(Location {
                    segment_id: Some(String::default()),
                    index: Some(usize::default()),
                }),
                row_indices: Some(Vec::default()),
                table_row_style: Some(TableRowStyle::default()),
                fields: Some(String::default()),
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
