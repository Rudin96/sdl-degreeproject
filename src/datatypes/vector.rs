use serde::{Serialize, ser::SerializeStruct, Deserialize};

// #[derive(Serialize, Deserialize, Debug)]
// pub struct Vector2 {
//     pub x: i32,
//     pub y: i32
// }

// impl Serialize for Vector2 {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//         where
//             S: serde::Serializer {
//         let mut s = serializer.serialize_struct("Vector2", 2)?;
//         s.serialize_field("x", &self.x)?;
//         s.serialize_field("y", &self.y)?;
//         s.end()
//     }
// }

// impl Deserialize for Vector2 {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//         where
//             D: serde::Deserializer<'de> {
                
//     }
// }