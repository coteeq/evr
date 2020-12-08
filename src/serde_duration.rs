use serde::de::{Deserializer, Error, Visitor};
use std::fmt;
use std::time::Duration;

pub fn deserialize_duration<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    struct DurationVisitor;

    impl<'de> Visitor<'de> for DurationVisitor {
        type Value = Duration;

        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "duration in secs")
        }

        fn visit_f32<E: Error>(self, v: f32) -> Result<Self::Value, E> {
            Ok(Duration::from_secs_f32(v))
        }

        fn visit_f64<E: Error>(self, v: f64) -> Result<Self::Value, E> {
            Ok(Duration::from_secs_f64(v))
        }
    }

    deserializer.deserialize_any(DurationVisitor)
}
