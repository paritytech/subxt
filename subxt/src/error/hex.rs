/// Display hex strings.
#[derive(PartialEq, Eq, Clone, Debug, PartialOrd, Ord)]
pub struct Hex(String);

impl<T: AsRef<[u8]>> From<T> for Hex {
    fn from(value: T) -> Self {
        Hex(hex::encode(value.as_ref()))
    }
}

impl std::fmt::Display for Hex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
