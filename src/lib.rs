pub mod v1;

#[cfg(test)]
mod tests {
    pub(crate) fn test_serde<
        T: std::fmt::Debug + PartialEq + serde::Serialize + serde::de::DeserializeOwned,
    >(
        s: &str,
        v: T,
    ) -> anyhow::Result<()> {
        assert_eq!(serde_json::from_str::<'_, T>(s)?, v);
        assert_eq!(
            serde_json::from_str::<'_, serde_json::Value>(&serde_json::to_string(&v)?)?,
            serde_json::from_str::<'_, serde_json::Value>(s)?
        );
        Ok(())
    }
}
