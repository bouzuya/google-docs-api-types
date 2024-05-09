/// <https://developers.google.com/docs/api/reference/rest/v1/documents/request#imagereplacemethod>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ImageReplaceMethod {
    #[default]
    ImageReplaceMethodUnspecified,
    CenterCrop,
}

#[cfg(test)]
mod tests {
    use crate::tests::test_serde;

    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        for (s, v) in [
            (
                r#""IMAGE_REPLACE_METHOD_UNSPECIFIED""#,
                ImageReplaceMethod::ImageReplaceMethodUnspecified,
            ),
            (r#""CENTER_CROP""#, ImageReplaceMethod::CenterCrop),
        ] {
            test_serde(s, v)?;
        }
        Ok(())
    }
}
