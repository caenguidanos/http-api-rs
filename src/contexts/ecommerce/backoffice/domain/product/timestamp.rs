use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct ProductTimeStamp(chrono::DateTime<chrono::offset::Utc>);

impl ProductTimeStamp {
    pub fn to_primitive(self) -> chrono::DateTime<chrono::offset::Utc> {
        let _e = tracing::debug_span!("Transform ProductTimeStamp to primitive").entered();

        self.0
    }
}

impl Default for ProductTimeStamp {
    fn default() -> Self {
        let _e = tracing::debug_span!("New ProductTimeStamp").entered();

        Self(chrono::Utc::now())
    }
}

impl Display for ProductTimeStamp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let _e = tracing::debug_span!("Display ProductTimeStamp").entered();

        write!(f, "{}", self.0.to_rfc3339_opts(chrono::SecondsFormat::Millis, false))
    }
}

impl From<chrono::DateTime<chrono::offset::Utc>> for ProductTimeStamp {
    fn from(value: chrono::DateTime<chrono::offset::Utc>) -> Self {
        let _e = tracing::debug_span!("Cast ProductTimeStamp from chrono::DateTime").entered();

        Self(value)
    }
}
