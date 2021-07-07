use std::{fmt, str::FromStr};

/// QQ 号码
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Serialize, Deserialize, Hash,
)]
#[repr(transparent)]
pub struct QQ(pub u64);

impl fmt::Display for QQ {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for QQ {
    type Err = <u64 as FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(QQ(s.parse()?))
    }
}

impl<T> From<T> for QQ
where
    T: Into<u64>,
{
    fn from(t: T) -> Self {
        let t: u64 = t.into();
        QQ(t)
    }
}
