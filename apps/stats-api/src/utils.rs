pub(crate) mod serde_date_time_fixed_offset {
    use serde::de::Error as _;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    use db::DateTime as DateTimeFixedOffset;

    pub fn serialize<S>(value: &DateTimeFixedOffset, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        value.to_rfc3339().serialize(serializer)
    }
    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTimeFixedOffset, D::Error>
    where
        D: Deserializer<'de>,
    {
        let rfc3339 = String::deserialize(deserializer)?;
        let datetime =
            DateTimeFixedOffset::parse_from_rfc3339(&rfc3339).map_err(D::Error::custom)?;

        Ok(datetime)
    }
}
