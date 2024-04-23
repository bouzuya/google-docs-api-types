use crate::v1::documents::request::BulletGlyphPreset;
use crate::v1::documents::Range;

/// <https://developers.google.com/docs/api/reference/rest/v1/documents/request#createparagraphbulletsrequest>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateParagraphBulletsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<Range>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bullet_preset: Option<BulletGlyphPreset>,
}
