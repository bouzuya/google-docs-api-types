/// <https://developers.google.com/docs/api/reference/rest/v1/documents/request#bulletglyphpreset>
#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Deserialize,
    serde::Serialize,
)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BulletGlyphPreset {
    #[default]
    BulletGlyphPresetUnspecified,
    BulletDiscCircleSquare,
    BulletDiamondxArrow3dSquare,
    BulletCheckbox,
    BulletArrowDiamondDisc,
    BulletStarCircleSquare,
    BulletArrow3dCircleSquare,
    BulletLefttriangleDiamondDisc,
    BulletDiamondxHollowdiamondSquare,
    BulletDiamondCircleSquare,
    NumberedDecimalAlphaRoman,
    NumberedDecimalAlphaRomanParens,
    NumberedDecimalNested,
    NumberedUpperalphaAlphaRoman,
    NumberedUpperromanUpperalphaDecimal,
    NumberedZerodecimalAlphaRoman,
}

#[cfg(test)]
mod tests {
    use crate::tests::test_serde;

    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        for (s, v) in [
            (
                r#""BULLET_GLYPH_PRESET_UNSPECIFIED""#,
                BulletGlyphPreset::BulletGlyphPresetUnspecified,
            ),
            (
                r#""BULLET_DISC_CIRCLE_SQUARE""#,
                BulletGlyphPreset::BulletDiscCircleSquare,
            ),
            (
                r#""BULLET_DIAMONDX_ARROW3D_SQUARE""#,
                BulletGlyphPreset::BulletDiamondxArrow3dSquare,
            ),
            (r#""BULLET_CHECKBOX""#, BulletGlyphPreset::BulletCheckbox),
            (
                r#""BULLET_ARROW_DIAMOND_DISC""#,
                BulletGlyphPreset::BulletArrowDiamondDisc,
            ),
            (
                r#""BULLET_STAR_CIRCLE_SQUARE""#,
                BulletGlyphPreset::BulletStarCircleSquare,
            ),
            (
                r#""BULLET_ARROW3D_CIRCLE_SQUARE""#,
                BulletGlyphPreset::BulletArrow3dCircleSquare,
            ),
            (
                r#""BULLET_LEFTTRIANGLE_DIAMOND_DISC""#,
                BulletGlyphPreset::BulletLefttriangleDiamondDisc,
            ),
            (
                r#""BULLET_DIAMONDX_HOLLOWDIAMOND_SQUARE""#,
                BulletGlyphPreset::BulletDiamondxHollowdiamondSquare,
            ),
            (
                r#""BULLET_DIAMOND_CIRCLE_SQUARE""#,
                BulletGlyphPreset::BulletDiamondCircleSquare,
            ),
            (
                r#""NUMBERED_DECIMAL_ALPHA_ROMAN""#,
                BulletGlyphPreset::NumberedDecimalAlphaRoman,
            ),
            (
                r#""NUMBERED_DECIMAL_ALPHA_ROMAN_PARENS""#,
                BulletGlyphPreset::NumberedDecimalAlphaRomanParens,
            ),
            (
                r#""NUMBERED_DECIMAL_NESTED""#,
                BulletGlyphPreset::NumberedDecimalNested,
            ),
            (
                r#""NUMBERED_UPPERALPHA_ALPHA_ROMAN""#,
                BulletGlyphPreset::NumberedUpperalphaAlphaRoman,
            ),
            (
                r#""NUMBERED_UPPERROMAN_UPPERALPHA_DECIMAL""#,
                BulletGlyphPreset::NumberedUpperromanUpperalphaDecimal,
            ),
            (
                r#""NUMBERED_ZERODECIMAL_ALPHA_ROMAN""#,
                BulletGlyphPreset::NumberedZerodecimalAlphaRoman,
            ),
        ] {
            test_serde(s, v)?;
        }
        Ok(())
    }
}
