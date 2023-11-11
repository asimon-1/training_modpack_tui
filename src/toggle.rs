use serde::ser::Serializer;
use serde::Serialize;

#[derive(Clone, Copy)]
pub struct Toggle<'a> {
    pub title: &'a str,
    pub value: u8,
    pub max: u8,
}

impl Serialize for Toggle<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u8(self.value)
    }
}
