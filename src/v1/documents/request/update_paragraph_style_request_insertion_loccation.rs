use crate::v1::documents::Range;

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum UpdateParagraphStyleRequestInsertionLocation {
    Range(Range),
}
