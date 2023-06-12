use std::fmt::{Display, Formatter};

// https://www.postgresql.org/docs/current/errcodes-appendix.html
pub enum Codes {
    UniqueViolation,
}

impl Display for Codes {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UniqueViolation => write!(f, "23505"),
        }
    }
}
