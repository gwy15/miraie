use std::fmt;

/// QQ 号码
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct QQ(u64);

impl fmt::Display for QQ {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
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
