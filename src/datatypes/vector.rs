use serde::{Serialize, ser::SerializeStruct};

pub struct Vector2 {
    pub x: i16,
    pub y: i16
}

impl Serialize for Vector2 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        let mut s = serializer.serialize_struct("Vector2", 2)?;
        s.serialize_field("x", &self.x)?;
        s.serialize_field("y", &self.y)?;
        s.end()
    }
}
