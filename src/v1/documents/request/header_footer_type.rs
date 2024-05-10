/// <https://developers.google.com/docs/api/reference/rest/v1/documents#alignment>
#[derive(Clone, Debug, Default, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HeaderFooterType {
    #[default]
    HeaderFooterTypeUnspecified,
    Default,
}

#[cfg(test)]
mod tests {
    use crate::tests::test_serde;

    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        for (s, v) in [
            (
                r#""HEADER_FOOTER_TYPE_UNSPECIFIED""#,
                HeaderFooterType::HeaderFooterTypeUnspecified,
            ),
            (r#""DEFAULT""#, HeaderFooterType::Default),
        ] {
            test_serde(s, v)?;
        }
        Ok(())
    }
}
